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
    /// Retorno sobre el capital total arriesgado (total_pnl / total_capital_risked)
    pub return_on_risk: f64,

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

    // Trade Duration
    /// Duración promedio de todos los trades (en milisegundos)
    pub average_trade_duration_ms: f64,
    /// Duración promedio de trades ganadores (en milisegundos)
    pub average_winning_trade_duration_ms: f64,
    /// Duración promedio de trades perdedores (en milisegundos)
    pub average_losing_trade_duration_ms: f64,

    // Drawdown (mejoras)
    /// Máximo drawdown como porcentaje (0.0 - 1.0)
    pub max_drawdown_percent: f64,

    // Profit/Loss totals
    /// Suma total de todos los trades ganadores
    pub total_profit: f64,
    /// Suma total de todos los trades perdedores (valor absoluto)
    pub total_loss: f64,

    // Streaks
    /// Racha máxima de trades ganadores consecutivos
    pub max_consecutive_wins: usize,
    /// Racha máxima de trades perdedores consecutivos
    pub max_consecutive_losses: usize,

    // Trading frequency
    /// Promedio de trades por mes
    pub trades_per_month: f64,
    /// Promedio de trades por año
    pub trades_per_year: f64,

    // Exit reasons (nuevos)
    /// Número de trades cerrados por Stop Loss
    pub stop_loss_exits: usize,
    /// Número de trades cerrados por Take Profit
    pub take_profit_exits: usize,
    /// Número de trades cerrados por señal de indicador
    pub signal_exits: usize,
    /// Número de trades cerrados al final de datos
    pub end_of_data_exits: usize,
}

impl Default for BacktestMetrics {
    fn default() -> Self {
        Self {
            total_return: 0.0,
            annualized_return: 0.0,
            sharpe_ratio: 0.0,
            sortino_ratio: 0.0,
            return_on_risk: 0.0,
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
            average_trade_duration_ms: 0.0,
            average_winning_trade_duration_ms: 0.0,
            average_losing_trade_duration_ms: 0.0,
            max_drawdown_percent: 0.0,
            total_profit: 0.0,
            total_loss: 0.0,
            max_consecutive_wins: 0,
            max_consecutive_losses: 0,
            trades_per_month: 0.0,
            trades_per_year: 0.0,
            stop_loss_exits: 0,
            take_profit_exits: 0,
            signal_exits: 0,
            end_of_data_exits: 0,
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

