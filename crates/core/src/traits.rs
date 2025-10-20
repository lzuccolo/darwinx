//! Traits fundamentales del sistema

pub mod exchange;
pub mod market_data;
pub mod risk_manager;
pub mod strategy;

// Re-exports
pub use exchange::Exchange;
pub use market_data::MarketData;
pub use risk_manager::RiskManager;
pub use strategy::Strategy;