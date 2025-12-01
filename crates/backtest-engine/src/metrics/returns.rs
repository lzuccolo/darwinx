//! Métricas de retorno

/// Calcula el retorno total
pub fn calculate_total_return(initial_balance: f64, final_balance: f64) -> f64 {
    (final_balance - initial_balance) / initial_balance
}

/// Calcula el retorno anualizado
pub fn calculate_annualized_return(
    total_return: f64,
    days: f64,
) -> f64 {
    if days <= 0.0 {
        return 0.0;
    }
    let years = days / 365.0;
    if years <= 0.0 {
        return 0.0;
    }
    (1.0 + total_return).powf(1.0 / years) - 1.0
}

/// Calcula el Sharpe Ratio
pub fn calculate_sharpe_ratio(returns: &[f64], risk_free_rate: f64) -> f64 {
    if returns.is_empty() {
        return 0.0;
    }

    let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;
    let excess_return = mean_return - risk_free_rate;

    let variance = returns
        .iter()
        .map(|r| (r - mean_return).powi(2))
        .sum::<f64>()
        / returns.len() as f64;

    let std_dev = variance.sqrt();

    if std_dev == 0.0 {
        return 0.0;
    }

    excess_return / std_dev
}

/// Calcula el Sortino Ratio
pub fn calculate_sortino_ratio(returns: &[f64], risk_free_rate: f64) -> f64 {
    if returns.is_empty() {
        return 0.0;
    }

    let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;
    let excess_return = mean_return - risk_free_rate;

    // Solo consideramos desviaciones negativas
    let downside_variance = returns
        .iter()
        .filter(|r| **r < 0.0)
        .map(|r| r.powi(2))
        .sum::<f64>()
        / returns.len() as f64;

    let downside_std = downside_variance.sqrt();

    if downside_std == 0.0 {
        return 0.0;
    }

    excess_return / downside_std
}

