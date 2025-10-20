//! # DarwinX Indicators
//!
//! Librería de indicadores técnicos para análisis de mercado.
//!
//! Todos los indicadores son funciones puras que operan sobre slices
//! de precios, permitiendo máxima eficiencia y reutilización.

pub mod trend;
pub mod momentum;
pub mod volatility;
pub mod volume;

// Re-exports de indicadores más comunes
pub use trend::{ema, sma, wma, vwma};
pub use momentum::{rsi, macd, stochastic, roc};
pub use volatility::{atr, bollinger, keltner};
pub use volume::{obv, mfi, vwap};
