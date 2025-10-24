⚡ Backtest Engine - Arquitectura Dual
🔥 1. Polars Engine (Vectorizado)
// crates/backtest-engine/src/polars_engine/
├── vectorized.rs     // Motor principal vectorizado
├── parallel.rs       // Ejecución paralela con Rayon
└── optimizer.rs      // Optimización de queries
Características:

✅ Vectorizado - Operaciones en columnas completas
✅ Lazy evaluation - Optimización automática de queries
✅ Paralelización automática con Rayon
✅ Throughput: 10,000+ estrategias/hora
✅ Uso: Backtest masivo, análisis estadístico

🎯 2. Event-Driven Engine (Simulación realista)
// crates/backtest-engine/src/event_driven/
├── engine.rs         // Motor principal event-driven
├── order_book.rs     // Simulación de order book
└── execution.rs      // Ejecución tick-by-tick
Características:

✅ Tick-by-tick - Simulación granular
✅ Order book simulation - Slippage realista
✅ Latencia simulada - Más realista
✅ Throughput: ~100 estrategias/hora
✅ Uso: Validación final, estrategias complejas

🏗️ ESTRUCTURA COMPLETA
// crates/backtest-engine/src/
├── lib.rs
├── types.rs                    // BacktestResult, Trade, Config
├── metrics/
│   ├── returns.rs             // Sharpe, Sortino, Total Return
│   ├── risk.rs                // Max DD, Calmar, VaR
│   └── statistics.rs          // Win Rate, Profit Factor
├── polars_engine/
│   ├── vectorized.rs          // 🚀 Motor rápido
│   ├── parallel.rs            // Batch execution
│   └── optimizer.rs           // Query optimization
├── event_driven/              
│   ├── engine.rs              // 🎯 Motor preciso
│   ├── order_book.rs          // Simulación realista
│   └── execution.rs           // Tick-by-tick
└── batch/
    ├── scheduler.rs           // Job scheduling
    └── worker.rs              // Worker pool
🎯 PLAN DE IMPLEMENTACIÓN ACTUALIZADO
Fase 1: Polars Engine (Semana 1)
bash# Día 1-2: Estructura + Vectorized
- Crear estructura completa
- Implementar polars_engine/vectorized.rs
- Evaluación básica de estrategias

# Día 3-4: Parallel + Metrics  
- parallel.rs con Rayon
- Métricas completas (Sharpe, DD, etc.)

# Día 5: Integration
- Tests de integración
- Ejemplo funcional
Fase 2: Event-Driven Engine (Semana 2)
bash# Día 1-2: Event Engine
- event_driven/engine.rs
- Simulación tick-by-tick básica

# Día 3-4: Order Book
- order_book.rs simulation
- Slippage y latencia realistas

# Día 5: Batch System
- scheduler.rs + worker.rs
- Sistema completo funcionando
🚀 EMPEZAMOS POR...
Polars Engine primero porque:

✅ Más simple de implementar
✅ Da resultados rápidos (validación)
✅ Permite testear todo el pipeline
✅ Event-driven se construye sobre esta base

💻 IMPLEMENTACIÓN TÉCNICA
Cargo.toml actualizado:
[dependencies]
darwinx-core = { path = "../core" }
darwinx-indicators = { path = "../indicators" }
darwinx-strategy-generator = { path = "../strategy-generator" }
polars = { version = "0.41", features = ["lazy", "temporal"] }
rayon = "1.10"
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
API pública:
// Dos engines distintos
pub use polars_engine::PolarsBacktestEngine;
pub use event_driven::EventDrivenEngine;

// Unified interface
pub trait BacktestEngine {
    async fn run_backtest(&self, strategy: StrategyAST, data: &[Candle]) -> BacktestResult;
    async fn run_batch(&self, strategies: Vec<StrategyAST>, data: &[Candle]) -> Vec<BacktestResult>;
}