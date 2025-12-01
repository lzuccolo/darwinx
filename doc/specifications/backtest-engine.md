# ⚡ Backtest Engine - Especificación Completa

**Versión**: 2.0  
**Última Actualización**: Octubre 2025

## Principios de Diseño

### Responsabilidad Única
- El Backtest Engine solo simula la ejecución de trades
- NO calcula indicadores (eso es responsabilidad de las estrategias)
- NO evalúa condiciones (las estrategias ya vienen compiladas)
- SÍ gestiona: órdenes, posiciones, balance, comisiones, slippage

### Separación de Responsabilidades
```
Strategy Generator → Compiled Strategy → Backtest Engine → Results
     ↑                      ↑                ↑
  (Genera AST)         (Lógica ejecutable)  (Simula trades)
```

## Arquitectura Dual

### 1. Polars Engine (Vectorizado)

**Características**:
- ✅ Vectorizado: Procesa múltiples velas simultáneamente
- ✅ Lazy Evaluation: Optimización automática de operaciones
- ✅ Paralelización: Batch execution con Rayon
- ✅ Throughput: 10,000+ estrategias/hora
- ✅ Uso: Backtest masivo, análisis estadístico

**Estructura**:
```
crates/backtest-engine/src/polars_engine/
├── vectorized.rs     // Motor principal vectorizado
├── parallel.rs       // Ejecución paralela con Rayon
└── optimizer.rs      // Optimización de queries
```

### 2. Event-Driven Engine (Simulación Realista)

**Características**:
- ✅ Tick-by-tick: Simulación granular paso a paso
- ✅ Order Book: Simulación realista de slippage
- ✅ Latencia: Simula delays reales del mercado
- ✅ Throughput: ~100 estrategias/hora
- ✅ Uso: Validación final, estrategias complejas

**Estructura**:
```
crates/backtest-engine/src/event_driven/
├── engine.rs         // Motor principal event-driven
├── order_book.rs     // Simulación de order book
└── execution.rs      // Ejecución tick-by-tick
```

## Input/Output

### Input: Estrategia Compilada

```rust
trait Strategy {
    fn name(&self) -> &str;
    fn should_enter_long(&self, candles: &[Candle], index: usize) -> bool;
    fn should_exit_long(&self, candles: &[Candle], index: usize) -> bool;
    fn should_enter_short(&self, candles: &[Candle], index: usize) -> bool;
    fn should_exit_short(&self, candles: &[Candle], index: usize) -> bool;
    fn position_size(&self, balance: f64, price: f64) -> f64;
}
```

### Output: Resultado de Backtest

```rust
struct BacktestResult {
    strategy_name: String,
    metrics: BacktestMetrics,
    trades: Vec<Trade>,
    equity_curve: Vec<EquityPoint>,
    metadata: BacktestMetadata,
}
```

## Sistema de Métricas

### Estructura Modular
```
crates/backtest-engine/src/metrics/
├── returns.rs        // Sharpe, Sortino, Total Return
├── risk.rs           // Max DD, Calmar, VaR
└── statistics.rs     // Win Rate, Profit Factor
```

### Métricas Calculadas

```rust
struct BacktestMetrics {
    // Returns
    total_return: f64,
    annualized_return: f64,
    cagr: f64,
    
    // Risk
    sharpe_ratio: f64,
    sortino_ratio: f64,
    max_drawdown: f64,
    calmar_ratio: f64,
    
    // Trading Stats
    total_trades: usize,
    win_rate: f64,
    profit_factor: f64,
    average_trade: f64,
}
```

## Sistema de Batch Processing

### Estructura
```
crates/backtest-engine/src/batch/
├── scheduler.rs      // Job scheduling y distribución
└── worker.rs         // Worker pool para paralelización
```

## API Pública Unificada

```rust
#[async_trait::async_trait]
pub trait BacktestEngine {
    /// Ejecuta backtest individual
    async fn run_backtest(
        &self,
        strategy: &dyn Strategy,
        data: &[Candle],
        config: &BacktestConfig,
    ) -> Result<BacktestResult>;

    /// Ejecuta múltiples backtests en batch
    async fn run_batch(
        &self,
        strategies: Vec<Box<dyn Strategy>>,
        data: &[Candle],
        config: &BacktestConfig,
        progress_callback: Option<ProgressCallback>,
    ) -> Result<Vec<BacktestResult>>;
}
```

## Configuración

```rust
struct BacktestConfig {
    initial_balance: f64,     // Balance inicial
    commission_rate: f64,     // Comisión por trade
    slippage_bps: f64,        // Slippage en basis points
    max_positions: usize,     // Máx posiciones simultáneas
    risk_per_trade: f64,      // Riesgo por trade (%)
}
```

## Performance Targets

| Operación | Target |
|-----------|--------|
| Backtest 1 estrategia (Polars) | < 1 segundo (100k velas) |
| Backtest 1 estrategia (Event) | < 10 segundos (100k velas) |
| Backtest masivo 10k (Polars) | < 60 minutos |
| Memory usage | < 2GB por 100k velas |
| CPU usage (batch) | < 95% |

## Estructura de Archivos Completa

```
crates/backtest-engine/
├── Cargo.toml
├── src/
│   ├── lib.rs                    // API pública
│   ├── error.rs                  // Error handling
│   ├── types.rs                  // Tipos de datos
│   ├── metrics/
│   │   ├── returns.rs           // Métricas de retorno
│   │   ├── risk.rs              // Métricas de riesgo
│   │   └── statistics.rs        // Estadísticas de trading
│   ├── polars_engine.rs
│   ├── polars_engine/
│   │   ├── vectorized.rs        // Motor vectorizado
│   │   ├── parallel.rs          // Ejecución paralela
│   │   └── optimizer.rs         // Optimización de queries
│   ├── event_driven.rs
│   ├── event_driven/
│   │   ├── engine.rs            // Motor event-driven
│   │   ├── order_book.rs        // Simulación order book
│   │   └── execution.rs         // Ejecución tick-by-tick
│   └── batch.rs
│   └── batch/
│       ├── scheduler.rs         // Job scheduling
│       └── worker.rs            // Worker pool
├── tests/
│   ├── integration_tests.rs     // Tests de integración
│   └── performance_tests.rs     // Benchmarks
└── benches/
    └── backtest_performance.rs  // Criterion benchmarks
```

## Flujo de Trabajo Completo

```
1. Strategy Generator → Compile Strategy → CompiledStrategy
                                               ↓
2. CompiledStrategy + Historical Data → BacktestEngine
                                               ↓
3. BacktestEngine → Simulate Trades → BacktestResult
                                               ↓
4. BacktestResult → Calculate Metrics → Final Report
```

## Principios de Diseño Finales

- ✅ **Single Responsibility**: Cada componente tiene una función específica
- ✅ **Dependency Injection**: Estrategias compiladas se inyectan
- ✅ **Performance First**: Optimizado para throughput masivo
- ✅ **Testability**: Interfaces limpias para testing
- ✅ **Extensibility**: Fácil agregar nuevos motores o métricas
