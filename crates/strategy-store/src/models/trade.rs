// ============================================================================
// crates/strategy-store/src/models/trade.rs
// ============================================================================
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Trade {
    pub id: Option<i64>,
    pub backtest_result_id: i64,
    pub entry_time: String,
    pub exit_time: String,
    pub side: String,
    pub entry_price: f64,
    pub exit_price: f64,
    pub quantity: f64,
    pub pnl: f64,
    pub pnl_percent: f64,
}