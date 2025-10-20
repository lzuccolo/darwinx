//! Tipos fundamentales del sistema

pub mod candle;
pub mod order;
pub mod position;
pub mod signal;
pub mod timeframe;

// Re-exports
pub use candle::Candle;
pub use order::{Order, OrderSide};
pub use position::{Position, PositionSide};
pub use signal::Signal;
pub use timeframe::TimeFrame;