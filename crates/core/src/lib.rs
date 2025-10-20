pub mod types;
pub mod traits;

// Re-exports para conveniencia
pub use types::{Candle, Order, OrderSide, Position, PositionSide, Signal, TimeFrame};
pub use traits::{Exchange, MarketData, RiskManager, Strategy};