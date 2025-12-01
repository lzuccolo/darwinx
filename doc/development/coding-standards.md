# 🎯 Rust 2024 Edition Standards - DarwinX

**Última Actualización**: Diciembre 2024

> **⚠️ PRINCIPIO FUNDAMENTAL**: 
> **Código simple, modular, performante y robusto. NUNCA OLVIDAR.**

Este documento define los estándares de codificación y convenciones para el proyecto DarwinX usando Rust 2024 edition.

> **📐 Design Principles**: Para guías arquitectónicas y de diseño (principios SOLID, separación de responsabilidades, etc.), ver [ARCHITECTURE.md](../architecture/technical.md).

## 🗂️ Organización de Módulos (Rust 2024)

### Convención de Nombres de Módulos

**Regla: Los directorios de módulos principales usan nombres en singular.**

```rust
// ✅ CORRECTO - Singular para módulos principales
src/
├── database/      // Módulo principal de base de datos (singular)
├── exchange/       // Módulo principal de exchange (singular)
├── strategy/       // Módulo principal de estrategia (singular)
├── types/          // Módulo principal de tipos (singular)
└── utils/          // Módulo principal de utilidades (singular)

// ✅ CORRECTO - Plural para subdirectorios con múltiples implementaciones
strategy/
├── strategies/     // Múltiples implementaciones de estrategias (plural)
│   ├── momentum.rs
│   ├── pairs.rs
│   └── keltner15_rsi_stoch.rs
└── core/          // Traits y tipos core (singular)

// ❌ INCORRECTO
src/
├── databases/     // Debe ser singular
├── exchanges/     // Debe ser singular
└── strategies/    // Debe ser strategy (singular)
```

**Razonamiento**: Los módulos principales representan un concepto único (database, exchange, strategy), mientras que subdirectorios como `strategies/` contienen múltiples implementaciones de ese concepto.

### Regla: NO usar mod.rs

```rust
// ❌ FORMA ANTIGUA (pre-2018)
src/
├── types/
│   ├── mod.rs     // NO USAR
│   ├── asset.rs
│   └── market.rs

// ✅ FORMA NUEVA (Rust 2024)
src/
├── types.rs       // Solo declaraciones de módulos
├── types/
│   ├── asset.rs   // Implementación completa
│   └── market.rs  // Implementación completa
```

**Ejemplo Real en DarwinX**:

```rust
// crates/data/src/lib.rs
pub mod loader;
pub mod multi_timeframe;

// crates/data/src/loader.rs
pub mod csv;
pub mod parquet;

// crates/data/src/loader/csv.rs
// Implementación completa del loader CSV
```

### Estructura de lib.rs

```rust
// src/lib.rs - SOLO declaraciones de módulos
#![deny(unreachable_pub, private_in_public)]
#![warn(missing_docs, clippy::missing_docs_in_private_items)]

//! DarwinX - Ecosistema de trading algorítmico

pub mod types;
pub mod strategy;
pub mod exchange;
pub mod data;
pub mod engine;
pub mod utils;
```

### Archivos de Declaración de Módulos

```rust
// src/types.rs - SOLO declaraciones de submódulos
//! Definiciones de tipos core para el sistema de trading

pub mod asset;
pub mod market;
pub mod order;
pub mod portfolio;
```

## 📝 Convenciones de Nombres

### Nombres de Archivos

- Usar `snake_case` para todos los nombres de archivos: `asset.rs`, `market_data.rs` ✅
- No usar: `AssetType.rs`, `marketData.rs`, `order-book.rs` ❌

### Nombres de Tipos

- Usar `PascalCase` para tipos: `TradingStrategy`, `OrderType`, `DataFeed` ✅
- No usar: `trading_strategy`, `orderType` ❌

### Nombres de Funciones y Variables

- Usar `snake_case`: `calculate_z_score()`, `entry_threshold` ✅
- Constantes: `MAX_POSITION_SIZE` (UPPER_SNAKE_CASE) ✅

### Ejemplos Reales en DarwinX

```rust
// ✅ CORRECTO
pub struct MultiTimeframeContext { ... }
pub struct TimeframeSynchronizer { ... }
pub fn load_multi_csv(...) -> Result<...> { ... }

// ❌ INCORRECTO
pub struct multi_timeframe_context { ... }
pub struct TimeFrameSynchronizer { ... }  // Debería ser TimeframeSynchronizer
pub fn LoadMultiCsv(...) -> Result<...> { ... }
```

## 🏗️ Estructura de Crates

### Organización de Crates

```
crates/
├── core/              # Tipos y traits fundamentales
│   ├── src/
│   │   ├── lib.rs
│   │   ├── types.rs
│   │   ├── types/
│   │   │   ├── candle.rs
│   │   │   ├── order.rs
│   │   │   └── timeframe.rs
│   │   └── traits.rs
│   │   └── traits/
│   │       ├── strategy.rs
│   │       └── exchange.rs
│
├── data/              # Carga de datos y multi-timeframe
│   ├── src/
│   │   ├── lib.rs
│   │   ├── loader.rs
│   │   ├── loader/
│   │   │   ├── csv.rs
│   │   │   └── parquet.rs
│   │   └── multi_timeframe.rs
│   │   └── multi_timeframe/
│   │       ├── context.rs
│   │       ├── synchronizer.rs
│   │       └── cache.rs
│
└── backtest-engine/   # Motor de backtest
    ├── src/
    │   ├── lib.rs
    │   ├── types.rs
    │   ├── metrics.rs
    │   ├── metrics/
    │   │   ├── returns.rs
    │   │   ├── risk.rs
    │   │   └── statistics.rs
    │   └── polars_engine.rs
    │   └── polars_engine/
    │       └── vectorized.rs
```

## 🎨 Principios de Diseño de Código

### 1. Simplicidad

```rust
// ✅ SIMPLE Y CLARO
pub fn calculate_total_return(initial: f64, final_balance: f64) -> f64 {
    (final_balance - initial) / initial
}

// ❌ COMPLEJO E INNECESARIO
pub fn calculate_total_return(initial_balance: f64, final_balance: f64) -> f64 {
    if initial_balance == 0.0 {
        return 0.0;
    }
    let difference = final_balance - initial_balance;
    let result = difference / initial_balance;
    result
}
```

### 2. Modularidad

```rust
// ✅ MODULAR - Cada módulo tiene una responsabilidad clara
pub mod metrics {
    pub mod returns;    // Solo métricas de retorno
    pub mod risk;       // Solo métricas de riesgo
    pub mod statistics; // Solo estadísticas
}

// ❌ NO MODULAR - Todo mezclado
pub mod metrics {
    // Todo en un solo archivo gigante
}
```

### 3. Performance

```rust
// ✅ PERFORMANTE - Usa referencias cuando es posible
pub fn get_candle(&self, index: usize) -> Option<&Candle> {
    self.candles.get(index)
}

// ❌ INEFICIENTE - Clonación innecesaria
pub fn get_candle(&self, index: usize) -> Option<Candle> {
    self.candles.get(index).cloned()
}
```

### 4. Robustez

```rust
// ✅ ROBUSTO - Manejo de errores explícito
pub fn load(path: &str) -> Result<Vec<Candle>, BacktestError> {
    let df = ParquetReader::new(&mut file)
        .finish()
        .map_err(|e| BacktestError::DataError(anyhow::anyhow!("Failed to read: {}", e)))?;
    // ...
}

// ❌ FRÁGIL - Usa unwrap() o expect()
pub fn load(path: &str) -> Vec<Candle> {
    let df = ParquetReader::new(&mut file).finish().unwrap(); // ❌
    // ...
}
```

## 📚 Documentación

### Documentación de Módulos

```rust
//! Módulo para carga de datos multi-timeframe
//!
//! Este módulo proporciona funcionalidad para cargar y sincronizar
//! datos de múltiples timeframes simultáneamente.

pub mod context;
pub mod synchronizer;
pub mod cache;
```

### Documentación de Funciones Públicas

```rust
/// Carga múltiples timeframes desde archivos CSV y crea un contexto multi-timeframe
///
/// # Arguments
/// * `paths` - Mapa de timeframe -> path del archivo CSV
/// * `primary_timeframe` - Timeframe principal
///
/// # Example
/// ```rust
/// use darwinx_data::{MultiTimeframeLoader, CsvLoader};
/// use darwinx_core::TimeFrame;
/// use std::collections::HashMap;
///
/// let mut paths = HashMap::new();
/// paths.insert(TimeFrame::M5, "data/m5.csv");
/// paths.insert(TimeFrame::H1, "data/h1.csv");
///
/// let context = MultiTimeframeLoader::load_multi_csv(&paths, TimeFrame::M5)?;
/// ```
pub fn load_multi_csv(
    paths: &HashMap<TimeFrame, &str>,
    primary_timeframe: TimeFrame,
) -> anyhow::Result<MultiTimeframeContext> {
    // ...
}
```

## 🧪 Testing

### Organización de Tests

```rust
// ✅ CORRECTO - Tests en el mismo archivo o en tests/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_single_csv() {
        // ...
    }
}

// O en tests/integration_tests.rs para tests de integración
```

### Nombres de Tests

```rust
// ✅ DESCRIPTIVO
#[test]
fn test_load_multi_csv_with_different_timeframes() { ... }

// ❌ VAGO
#[test]
fn test1() { ... }
```

## ⚡ Performance Guidelines

### 1. Evitar Clonaciones Innecesarias

```rust
// ✅ Usa referencias
fn process_candles(candles: &[Candle]) { ... }

// ❌ Clonación innecesaria
fn process_candles(candles: Vec<Candle>) { ... }
```

### 2. Usa Polars para Operaciones Vectorizadas

```rust
// ✅ Vectorizado con Polars
let df = LazyFrame::scan_parquet(path, Default::default())?
    .collect()?;

// ❌ Loop manual
for row in rows {
    // procesar una por una
}
```

### 3. Pre-allocate cuando sea posible

```rust
// ✅ Pre-asignación
let mut candles = Vec::with_capacity(expected_size);

// ❌ Re-asignación múltiple
let mut candles = Vec::new();
```

## 🔒 Seguridad y Robustez

### 1. Nunca usar `unwrap()` en código de producción

```rust
// ✅ Manejo explícito de errores
match result {
    Ok(value) => value,
    Err(e) => return Err(BacktestError::DataError(e.into())),
}

// ❌ Unwrap peligroso
let value = result.unwrap();
```

### 2. Validar inputs

```rust
// ✅ Validación
pub fn new(initial_balance: f64) -> Result<Self, BacktestError> {
    if initial_balance <= 0.0 {
        return Err(BacktestError::ConfigError(
            "Initial balance must be positive".to_string(),
        ));
    }
    Ok(Self { initial_balance })
}
```

### 3. Usa tipos fuertes

```rust
// ✅ Tipo fuerte
pub struct Balance(f64);

impl Balance {
    pub fn new(value: f64) -> Result<Self, BacktestError> {
        if value <= 0.0 {
            return Err(BacktestError::ConfigError("Invalid balance".to_string()));
        }
        Ok(Self(value))
    }
}

// ❌ Tipo débil
pub fn process_balance(balance: f64) { ... } // Puede recibir valores inválidos
```

## 📋 Checklist de Revisión de Código

Antes de hacer commit, verificar:

- [ ] ✅ Código simple y claro
- [ ] ✅ Modular (una responsabilidad por módulo)
- [ ] ✅ Performante (sin clonaciones innecesarias, usa referencias)
- [ ] ✅ Robustez (manejo de errores explícito, sin unwrap())
- [ ] ✅ Nombres de módulos en singular para principales
- [ ] ✅ NO usa mod.rs
- [ ] ✅ Documentación en funciones públicas
- [ ] ✅ Tests para funcionalidad crítica
- [ ] ✅ Compila sin warnings
- [ ] ✅ Sigue convenciones de nombres (PascalCase para tipos, snake_case para funciones)

## 🎯 Resumen de Principios

1. **Simplicidad**: Código claro y directo, evita complejidad innecesaria
2. **Modularidad**: Cada módulo tiene una responsabilidad única
3. **Performance**: Optimizado para velocidad y eficiencia
4. **Robustez**: Manejo de errores explícito, validación de inputs
5. **Rust 2024**: Usa las convenciones modernas (sin mod.rs, estructura clara)

---

**Recuerda**: Código simple, modular, performante y robusto. **NUNCA OLVIDAR.**

