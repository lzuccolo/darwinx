# DarwinX - Arquitectura Técnica

**Versión**: 2.0  
**Última Actualización**: Octubre 2025

## Visión General

DarwinX es un ecosistema completo de trading con generación automática de estrategias, backtest vectorizado, y arquitectura cliente-servidor. Diseñado para escalabilidad y automatización.

## Arquitectura de Crates

```
darwinx/
├── crates/core/              # Tipos y traits fundamentales
├── crates/indicators/        # Indicadores técnicos
├── crates/data/             # Carga de datos y multi-timeframe
├── crates/strategy-store/    # Persistencia (PostgreSQL)
├── crates/strategy-generator/# Generación automática de estrategias
├── crates/api-proto/        # Protocol Buffers (gRPC)
└── (futuros: backtest-engine, optimizer, runner-live, etc.)
```

### `crates/core/`
Tipos y traits base:
- `types/`: `Candle`, `Order`, `Position`, `Signal`, `TimeFrame`
- `traits/`: `Strategy`, `MarketData`, `RiskManager`, `Exchange`

### `crates/indicators/`
Sistema de indicadores con registro automático:
- `registry.rs`: Registro centralizado de indicadores
- `trend/`: SMA, EMA, WMA, VWMA
- `momentum/`: RSI, MACD, Stochastic, ROC
- `volatility/`: Bollinger, ATR, Keltner
- `volume/`: OBV, MFI, VWAP

**Característica**: Auto-registro mediante macros y `ctor`.

### `crates/data/`
Carga de datos y multi-timeframe:
- `loader/`: `CsvLoader`, `ParquetLoader`
- `multi_timeframe/`:
  - `synchronizer.rs`: Sincronización de timeframes
  - `context.rs`: Contexto multi-timeframe
  - `cache.rs`: Cache de datos
  - `alignment.rs`: Alineación temporal

**Característica clave**: Sincronización avanzada de múltiples timeframes.

### `crates/strategy-store/`
Persistencia en PostgreSQL:
- `models/`: Modelos de estrategias, backtests, trades
- `repositories/`: Repositorios para acceso a datos
- `database.rs`: Conexión y migraciones

### `crates/strategy-generator/`
Generación automática de estrategias:
- `generator/`: Algoritmos (random, genetic)
- `ast/`: AST para representar estrategias
- `constraints.rs`: Validación y restricciones

### `crates/api-proto/`
Protocol Buffers para gRPC:
- `common.proto`: Tipos comunes
- `strategy.proto`: Servicios de estrategias
- `backtest.proto`: Servicios de backtest
- `optimizer.proto`: Servicios de optimización
- `live.proto`: Servicios de ejecución en vivo

## Arquitectura Cliente-Servidor

```
┌─────────────┐         gRPC          ┌─────────────┐
│   Client   │ ◄──────────────────► │   Server    │
│  (CLI/GUI) │                       │  (Backend)  │
└─────────────┘                       └─────────────┘
                                             │
                                             ▼
                                      ┌─────────────┐
                                      │ PostgreSQL  │
                                      │  (Storage)  │
                                      └─────────────┘
```

## Backtest Engine (Planeado)

Arquitectura dual:

### 1. Polars Engine (Vectorizado)
- **Throughput**: 10,000+ estrategias/hora
- **Uso**: Backtest masivo, análisis estadístico
- **Características**:
  - Operaciones vectorizadas en columnas
  - Lazy evaluation
  - Paralelización automática con Rayon

### 2. Event-Driven Engine (Simulación realista)
- **Throughput**: ~100 estrategias/hora
- **Uso**: Validación final, estrategias complejas
- **Características**:
  - Tick-by-tick
  - Order book simulation
  - Slippage y latencia realistas

## Multi-Timeframe

Sistema avanzado de sincronización:

```rust
pub struct TimeframeSynchronizer;

impl TimeframeSynchronizer {
    pub fn sync_timeframes(
        data: &HashMap<TimeFrame, Vec<Candle>>,
        timestamp: i64,
    ) -> HashMap<TimeFrame, Candle> {
        // Sincroniza múltiples timeframes a un timestamp específico
    }
}
```

**Características**:
- Sincronización por timestamp
- Forward-fill de datos faltantes
- Compatibilidad entre timeframes

## Generación de Estrategias

Sistema de generación automática:

1. **Algoritmos**:
   - Random: Generación aleatoria
   - Genetic: Algoritmo genético
   - Grid: Búsqueda en grilla

2. **AST**: Representación de estrategias como árbol sintáctico

3. **Validación**: Constraints y reglas de validación

4. **Compilación**: Conversión a diferentes formatos (Rhai, Rust, Python, Freqtrade)

## Flujo de Datos

```
Data Loader (CSV/Parquet)
    ↓
Multi-Timeframe Synchronizer
    ↓
Strategy Generator / Loader
    ↓
Backtest Engine (Polars/Event-Driven)
    ↓
Results → Strategy Store (PostgreSQL)
    ↓
API Server (gRPC)
    ↓
Client (CLI/GUI)
```

## Características Clave

✅ **Backtest vectorizado**: Polars para throughput masivo  
✅ **Multi-timeframe**: Sincronización avanzada  
✅ **Generación automática**: Algoritmos genéticos y grid search  
✅ **Arquitectura escalable**: Cliente-servidor con gRPC  
✅ **Persistencia**: PostgreSQL para estrategias y resultados  
✅ **Indicadores extensibles**: Sistema de registro automático  

## Dependencias Principales

- `polars`: Procesamiento de datos vectorizado
- `tonic`: gRPC (Tonic)
- `sqlx`: PostgreSQL
- `rhai`: Scripting de estrategias
- `rayon`: Paralelización
- `tokio`: Runtime async

## Estado Actual

**Implementado**:
- ✅ Core types y traits
- ✅ Sistema de indicadores
- ✅ Data loaders (CSV, Parquet)
- ✅ Multi-timeframe synchronizer (95% completo)
- ✅ Strategy store (PostgreSQL)
- ✅ Strategy generator (completo con multi-TF)
- ✅ API proto (gRPC)

**Pendiente**:
- ⏳ Backtest engine (Polars/Event-driven)
- ⏳ Strategy converter
- ⏳ Optimizer
- ⏳ Runner live
- ⏳ API server
- ⏳ CLI client
- ⏳ GUI client

## Estructura de Datos

```
data/
├── raw/          # Datos crudos
└── processed/    # Datos procesados

strategies/
├── strategies.db  # SQLite local (desarrollo)
├── intermediate/  # Estrategias en formato intermedio
├── rhai/          # Estrategias en Rhai
└── compiled/      # Estrategias compiladas
```

## Ventajas

1. **Escalabilidad**: Arquitectura cliente-servidor permite múltiples clientes
2. **Automatización**: Generación automática de estrategias
3. **Rendimiento**: Backtest vectorizado con Polars
4. **Flexibilidad**: Soporte para múltiples formatos de estrategias
5. **Extensibilidad**: Sistema de indicadores extensible

## Limitaciones Actuales

- Complejidad alta (muchos crates, algunos no implementados)
- Backtest vectorizado requiere reescribir estrategias
- Overhead de gRPC para backtest local
- Dependencias pesadas (Polars, Tonic, SQLx)

## Futuras Mejoras

- [ ] Completar backtest engine
- [ ] Optimizador de parámetros
- [ ] Runner en vivo
- [ ] API server completo
- [ ] CLI y GUI clients
- [ ] Dashboard web

