use crate::trend::ema;
use crate::volatility::atr;

/// Calcula los Keltner Channels
/// Retorna (lower, middle, upper)
pub fn keltner_channels(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period: usize,
    multiplier: f64,
) -> Option<(f64, f64, f64)> {
    let middle = ema(close, period)?;
    let atr_val = atr(high, low, close, period)?;
    
    Some((
        middle - multiplier * atr_val,
        middle,
        middle + multiplier * atr_val,
    ))
}
