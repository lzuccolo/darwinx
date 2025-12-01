//! Métricas de riesgo

use crate::types::EquityPoint;

/// Calcula el máximo drawdown
pub fn calculate_max_drawdown(equity_curve: &[EquityPoint]) -> f64 {
    if equity_curve.is_empty() {
        return 0.0;
    }

    let mut max_equity = equity_curve[0].balance;
    let mut max_dd = 0.0;

    for point in equity_curve {
        if point.balance > max_equity {
            max_equity = point.balance;
        }

        let drawdown = (max_equity - point.balance) / max_equity;
        if drawdown > max_dd {
            max_dd = drawdown;
        }
    }

    max_dd
}

/// Calcula la duración del máximo drawdown (en velas)
pub fn calculate_max_drawdown_duration(equity_curve: &[EquityPoint]) -> u64 {
    if equity_curve.is_empty() {
        return 0;
    }

    let mut max_equity = equity_curve[0].balance;
    let mut max_equity_index = 0;
    let mut max_dd_duration = 0;
    let mut current_dd_duration = 0;

    for (i, point) in equity_curve.iter().enumerate() {
        if point.balance > max_equity {
            max_equity = point.balance;
            max_equity_index = i;
            current_dd_duration = 0;
        } else if point.balance < max_equity {
            current_dd_duration = i - max_equity_index;
            if current_dd_duration > max_dd_duration {
                max_dd_duration = current_dd_duration;
            }
        }
    }

    max_dd_duration as u64
}

/// Calcula el Calmar Ratio
pub fn calculate_calmar_ratio(annualized_return: f64, max_drawdown: f64) -> f64 {
    if max_drawdown == 0.0 {
        return 0.0;
    }
    annualized_return / max_drawdown
}

/// Calcula el Value at Risk (VaR) al 95%
pub fn calculate_var_95(returns: &[f64]) -> f64 {
    if returns.is_empty() {
        return 0.0;
    }

    let mut sorted_returns = returns.to_vec();
    sorted_returns.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let index = (sorted_returns.len() as f64 * 0.05) as usize;
    -sorted_returns[index.min(sorted_returns.len() - 1)]
}

