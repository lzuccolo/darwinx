// ============================================================================
// crates/strategy-store/src/models/backtest_result.rs
// ============================================================================
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BacktestResult {
    pub id: Option<i64>,
    pub strategy_id: i64,
    pub dataset: String,
    pub timeframe: String,
    pub start_date: String,
    pub end_date: String,
    pub total_return: f64,
    pub sharpe_ratio: f64,
    pub sortino_ratio: Option<f64>,
    pub max_drawdown: f64,
    pub win_rate: f64,
    pub profit_factor: Option<f64>,
    pub total_trades: i32,
    pub tested_at: Option<String>,
    // Extended metrics
    pub annualized_return: Option<f64>,
    pub max_drawdown_percent: Option<f64>,
    pub total_profit: Option<f64>,
    pub total_loss: Option<f64>,
    pub max_consecutive_wins: Option<i32>,
    pub max_consecutive_losses: Option<i32>,
    pub trades_per_month: Option<f64>,
    pub trades_per_year: Option<f64>,
    pub stop_loss_exits: Option<i32>,
    pub take_profit_exits: Option<i32>,
    pub signal_exits: Option<i32>,
    pub end_of_data_exits: Option<i32>,
    pub composite_score: Option<f64>, // Weighted score from ranking
}
