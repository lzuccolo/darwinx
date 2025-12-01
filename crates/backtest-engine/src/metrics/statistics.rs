//! Estadísticas de trading

use crate::types::Trade;

/// Calcula el win rate
pub fn calculate_win_rate(trades: &[Trade]) -> f64 {
    if trades.is_empty() {
        return 0.0;
    }

    let winning = trades.iter().filter(|t| t.pnl > 0.0).count();
    winning as f64 / trades.len() as f64
}

/// Calcula el profit factor
pub fn calculate_profit_factor(trades: &[Trade]) -> f64 {
    let total_profit: f64 = trades.iter().filter(|t| t.pnl > 0.0).map(|t| t.pnl).sum();
    let total_loss: f64 = trades.iter().filter(|t| t.pnl < 0.0).map(|t| t.pnl.abs()).sum();

    if total_loss == 0.0 {
        return if total_profit > 0.0 { f64::INFINITY } else { 0.0 };
    }

    total_profit / total_loss
}

/// Calcula el promedio de ganancias
pub fn calculate_average_win(trades: &[Trade]) -> f64 {
    let wins: Vec<f64> = trades.iter().filter(|t| t.pnl > 0.0).map(|t| t.pnl).collect();
    if wins.is_empty() {
        return 0.0;
    }
    wins.iter().sum::<f64>() / wins.len() as f64
}

/// Calcula el promedio de pérdidas
pub fn calculate_average_loss(trades: &[Trade]) -> f64 {
    let losses: Vec<f64> = trades.iter().filter(|t| t.pnl < 0.0).map(|t| t.pnl.abs()).collect();
    if losses.is_empty() {
        return 0.0;
    }
    losses.iter().sum::<f64>() / losses.len() as f64
}

/// Calcula la ganancia más grande
pub fn calculate_largest_win(trades: &[Trade]) -> f64 {
    trades
        .iter()
        .filter(|t| t.pnl > 0.0)
        .map(|t| t.pnl)
        .fold(0.0, f64::max)
}

/// Calcula la pérdida más grande
pub fn calculate_largest_loss(trades: &[Trade]) -> f64 {
    trades
        .iter()
        .filter(|t| t.pnl < 0.0)
        .map(|t| t.pnl.abs())
        .fold(0.0, f64::max)
}

/// Calcula la expectativa
pub fn calculate_expectancy(trades: &[Trade]) -> f64 {
    if trades.is_empty() {
        return 0.0;
    }

    let win_rate = calculate_win_rate(trades);
    let avg_win = calculate_average_win(trades);
    let avg_loss = calculate_average_loss(trades);

    (win_rate * avg_win) - ((1.0 - win_rate) * avg_loss)
}

/// Calcula el recovery factor
pub fn calculate_recovery_factor(total_profit: f64, max_drawdown: f64) -> f64 {
    if max_drawdown == 0.0 {
        return 0.0;
    }
    total_profit / max_drawdown
}

