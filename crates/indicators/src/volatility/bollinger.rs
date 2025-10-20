use crate::trend::sma;

/// Calcula las Bandas de Bollinger
/// Retorna (lower, middle, upper)
pub fn bollinger_bands(data: &[f64], period: usize, std_dev: f64) -> Option<(f64, f64, f64)> {
    let sma_val = sma(data, period)?;
    
    let variance: f64 = data.iter()
        .rev()
        .take(period)
        .map(|x| (x - sma_val).powi(2))
        .sum::<f64>() / period as f64;
    
    let std = variance.sqrt();
    
    Some((
        sma_val - std_dev * std,
        sma_val,
        sma_val + std_dev * std,
    ))
}
