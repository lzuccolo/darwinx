//! # DarwinX Strategy Store
//!
//! Sistema de persistencia para estrategias y resultados de backtest

pub mod models;
pub mod repositories;
pub mod database;

// Re-exports
pub use models::{BacktestResult, Strategy, Trade};
pub use repositories::{BacktestRepository, StrategyRepository};
pub use database::init_sqlite;