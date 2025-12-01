//! Backtest Engine - Motor de simulación de trading
//!
//! Este crate proporciona dos motores de backtest:
//! 1. **Polars Engine**: Motor vectorizado para backtest masivo
//! 2. **Event-Driven Engine**: Motor tick-by-tick para simulación realista
//!
//! # Principios de Diseño
//!
//! - **Single Responsibility**: Solo simula ejecución de trades
//! - **Dependency Injection**: Estrategias compiladas se inyectan
//! - **Performance First**: Optimizado para throughput masivo
//! - **Testability**: Interfaces limpias para testing

pub mod error;
pub mod types;
pub mod data_provider;
pub mod metrics;
pub mod polars_engine;
pub mod event_driven;
pub mod config;

// Re-exports
pub use error::BacktestError;
pub use types::{BacktestResult, BacktestMetrics, Trade, EquityPoint, BacktestMetadata};
pub use data_provider::{DataProvider, SingleTimeFrameProvider, MultiTimeFrameProvider};
pub use config::BacktestConfig;
pub use polars_engine::PolarsBacktestEngine;
pub use polars_engine::vectorized::{Strategy, BacktestEngine};
pub use event_driven::EventDrivenBacktestEngine;
