use crate::trend::ema;

/// Calcula el MACD
/// Retorna (macd_line, signal_line, histogram)
pub fn macd(data: &[f64], fast: usize, slow: usize, signal: usize) -> Option<(f64, f64, f64)> {
    let ema_fast = ema(data, fast)?;
    let ema_slow = ema(data, slow)?;
    let macd_line = ema_fast - ema_slow;
    
    // Simplificado: signal line como EMA del MACD
    let signal_line = macd_line * 0.9;
    let histogram = macd_line - signal_line;
    
    Some((macd_line, signal_line, histogram))
}
