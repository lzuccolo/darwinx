# 🚀 Cómo Ejecutar el Generador de Estrategias

## 📋 Resumen

El generador de estrategias de DarwinX permite crear estrategias de trading automáticamente usando algoritmos genéticos o generación aleatoria. Actualmente hay dos implementaciones:

1. **`crates/strategy-generator/`** - Versión básica
2. **`crates/files/`** - Versión completa con multi-timeframe (recomendada)

## 🎯 Uso Básico

### Opción 1: Ejecutar el Ejemplo Incluido

```bash
# Desde la raíz del proyecto
cargo run --package darwinx-generator --example run_generator
```

Esto generará 10 estrategias y mostrará estadísticas.

### Opción 2: Desde Código Rust

#### Generación Aleatoria Simple

```rust
use darwinx_generator::*;

// Crear generador
let generator = RandomGenerator::new();

// Generar una estrategia
let strategy = generator.generate("Mi Estrategia".to_string());

println!("Estrategia generada: {}", strategy.name);
println!("Timeframe: {:?}", strategy.timeframe);
println!("Condiciones entrada: {}", strategy.entry_rules.conditions.len());
```

#### Generación en Batch

```rust
// Generar múltiples estrategias
let strategies = generator.generate_batch(100);

println!("Generadas {} estrategias", strategies.len());
for (i, strategy) in strategies.iter().enumerate() {
    println!("{}. {} - {} condiciones", 
             i + 1, 
             strategy.name, 
             strategy.complexity());
}
```

### Opción 3: Versión Multi-Timeframe (crates/files - más avanzada)

```rust
use darwinx_strategy_generator::*; // Si está en files/
use darwinx_core::TimeFrame;

// Crear generador con configuración
let mut generator = RandomGenerator::default_with_seed(Some(42))
    .with_timeframe_distribution(TimeframeDistribution::multi_timeframe_focused());

// Generar estrategias multi-timeframe
let strategies = generator.generate_batch(100, "MultiTF", TimeFrame::H1);

// Filtrar solo multi-timeframe
let multi_tf_count = strategies.iter()
    .filter(|s| s.is_multi_timeframe())
    .count();

println!("Generadas {} estrategias multi-timeframe de {}", 
         multi_tf_count, strategies.len());
```

## 📝 Ejemplo Completo

### Ver el Ejemplo Completo

El ejemplo ya está creado en `examples/run_generator.rs`. Puedes verlo o modificarlo:

```rust
use darwinx_strategy_generator::*;
use darwinx_core::TimeFrame;

fn main() {
    println!("🧬 DarwinX Strategy Generator");
    println!("=============================\n");

    // 1. Crear generador
    let generator = RandomGenerator::new();

    // 2. Generar estrategias
    println!("Generando 10 estrategias...\n");
    let strategies = generator.generate_batch(10);

    // 3. Mostrar resultados
    for (i, strategy) in strategies.iter().enumerate() {
        println!("{}. {}", i + 1, strategy.name);
        println!("   Timeframe: {:?}", strategy.primary_timeframe);
        println!("   Entrada: {} condiciones", strategy.entry_rules.conditions.len());
        println!("   Salida: {} condiciones", strategy.exit_rules.conditions.len());
        println!("   Complejidad: {}", strategy.complexity());
        println!();
    }

    // 4. Estadísticas
    let avg_complexity: f64 = strategies.iter()
        .map(|s| s.complexity() as f64)
        .sum::<f64>() / strategies.len() as f64;
    
    println!("📊 Estadísticas:");
    println!("   Promedio complejidad: {:.2}", avg_complexity);
    println!("   Total estrategias: {}", strategies.len());
}
```

### Ejecutar

```bash
# Desde la raíz del proyecto
cargo run --example run_generator

# O compilar y ejecutar
cargo build --example run_generator
./target/debug/examples/run_generator
```

## 🔧 Configuración Avanzada

### Con Constraints

```rust
use darwinx_strategy_generator::*;

// Constraints estrictos (menos condiciones)
let generator = RandomGenerator::with_constraints(
    3,  // max_conditions
    2   // max_indicators
);

let strategy = generator.generate("Simple Strategy".to_string());
```

### Validación de Estrategias

```rust
use darwinx_strategy_generator::*;

let generator = RandomGenerator::new();
let strategy = generator.generate("Test".to_string());

// Validar estrategia
let validator = StrategyValidator::new(StrategyConstraints::default());
let is_valid = validator.validate(&strategy);

if is_valid {
    println!("✅ Estrategia válida!");
} else {
    println!("❌ Estrategia inválida");
}
```

## 🧪 Ejecutar Tests

```bash
# Tests del generador
cargo test --package darwinx-strategy-generator

# Tests específicos
cargo test --package darwinx-strategy-generator generator::random

# Con output
cargo test --package darwinx-strategy-generator -- --nocapture
```

## 📚 Ejemplos Incluidos

El crate `files/` incluye ejemplos completos en `examples.rs`:

```rust
use darwinx_strategy_generator::examples::*;

// Ejecutar todos los ejemplos
run_all_examples();

// O ejemplos individuales
example_golden_cross_multi_tf();
example_automatic_generation();
example_detailed_validation();
```

## 🎯 Integración con Backtest

Una vez generadas las estrategias, puedes usarlas con el Backtest Engine:

```rust
use darwinx_strategy_generator::*;
use darwinx_backtest_engine::*;
use darwinx_data::*;

// 1. Generar estrategia
let generator = RandomGenerator::new();
let strategy_ast = generator.generate("Test Strategy".to_string());

// 2. Convertir a Strategy trait (requiere implementación)
// let strategy = convert_ast_to_strategy(strategy_ast);

// 3. Cargar datos
let candles = CsvLoader::load("data/btcusdt_1h.csv")?;
let provider = SingleTimeFrameProvider::new(candles, TimeFrame::H1);

// 4. Ejecutar backtest
let engine = PolarsBacktestEngine::new();
let config = BacktestConfig::default();
// let result = engine.run_backtest(&strategy, &provider, &config).await?;
```

## ⚠️ Nota Importante

**Estado Actual**: El generador crea `StrategyAST` (Abstract Syntax Tree), pero para usar con el Backtest Engine necesitas:

1. **Strategy Converter** (pendiente) - Convierte AST a implementación ejecutable
2. O implementar manualmente el trait `Strategy` para cada AST

## 🔗 Referencias

- [Guía del Generador](./strategy-generator.md) - Documentación completa
- [Ejemplos en código](../crates/files/examples.rs) - Ejemplos prácticos
- [README del Generador](../crates/files/README.md) - Documentación técnica

