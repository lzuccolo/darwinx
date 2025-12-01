//! Tipos de datos para el Backtest Engine

use serde::{Deserialize, Serialize};
use darwinx_core::{Candle, Order, Position};

/// Resultado completo de un backtest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    /// Nombre de la estrategia
    pub strategy_name: String,
    /// Métricas calculadas
    pub metrics: BacktestMetrics,
    /// Lista de trades ejecutados
    pub trades: Vec<Trade>,
    /// Curva de equity (balance a lo largo del tiempo)
    pub equity_curve: Vec<EquityPoint>,
    /// Metadatos del backtest
    pub metadata: BacktestMetadata,
}

/// Métricas de performance del backtest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestMetrics {
    // Returns
    pub total_return: f64,
    pub annualized_return: f64,
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,

    // Risk
    pub max_drawdown: f64,
    pub max_drawdown_duration: u64, // en velas
    pub calmar_ratio: f64,
    pub var_95: f64, // Value at Risk al 95%

    // Statistics
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub average_win: f64,
    pub average_loss: f64,
    pub largest_win: f64,
    pub largest_loss: f64,

    // Additional
    pub expectancy: f64,
    pub recovery_factor: f64,
}

impl Default for BacktestMetrics {
    fn default() -> Self {
        Self {
            total_return: 0.0,
            annualized_return: 0.0,
            sharpe_ratio: 0.0,
            sortino_ratio: 0.0,
            max_drawdown: 0.0,
            max_drawdown_duration: 0,
            calmar_ratio: 0.0,
            var_95: 0.0,
            total_trades: 0,
            winning_trades: 0,
            losing_trades: 0,
            win_rate: 0.0,
            profit_factor: 0.0,
            average_win: 0.0,
            average_loss: 0.0,
            largest_win: 0.0,
            largest_loss: 0.0,
            expectancy: 0.0,
            recovery_factor: 0.0,
        }
    }
}

/// Trade ejecutado durante el backtest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Timestamp de entrada
    pub entry_timestamp: i64,
    /// Timestamp de salida
    pub exit_timestamp: i64,
    /// Precio de entrada
    pub entry_price: f64,
    /// Precio de salida
    pub exit_price: f64,
    /// Tamaño de la posición
    pub size: f64,
    /// Dirección (true = long, false = short)
    pub is_long: bool,
    /// P&L del trade
    pub pnl: f64,
    /// Comisiones pagadas
    pub commission: f64,
    /// Slippage incurrido
    pub slippage: f64,
    /// Razón de salida
    pub exit_reason: String,
}

/// Punto en la curva de equity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityPoint {
    /// Timestamp
    pub timestamp: i64,
    /// Balance en este punto
    pub balance: f64,
    /// Drawdown en este punto
    pub drawdown: f64,
}

/// Metadatos del backtest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestMetadata {
    /// Fecha de inicio del backtest
    pub start_date: i64,
    /// Fecha de fin del backtest
    pub end_date: i64,
    /// Número total de velas procesadas
    pub total_candles: usize,
    /// Balance inicial
    pub initial_balance: f64,
    /// Balance final
    pub final_balance: f64,
    /// Configuración usada
    pub config: BacktestConfig,
}

/// Configuración del backtest (definida en config.rs)
pub use crate::config::BacktestConfig;

