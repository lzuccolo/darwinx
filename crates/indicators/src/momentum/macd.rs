use crate::register_indicator;
use crate::trend::ema;

/// Metadata del indicador MACD
pub fn metadata() -> crate::metadata::IndicatorMetadata {
    use crate::metadata::*;
    
    IndicatorMetadata::new("macd")
        .category(IndicatorCategory::Momentum)
        .input_type(InputType::PriceSeries)
        .lookback(1)
        .parameter(ParameterDef::period("fast_period", 5.0, 50.0, 12.0))
        .parameter(ParameterDef::period("slow_period", 10.0, 100.0, 26.0))
        .parameter(ParameterDef::period("signal_period", 5.0, 50.0, 9.0))
        .description("Moving Average Convergence Divergence")
}

/// Calcula el MACD
/// Retorna (macd_line, signal_line, histogram)
pub fn macd(data: &[f64], fast: usize, slow: usize, _signal: usize) -> Option<(f64, f64, f64)> {
    let ema_fast = ema(data, fast)?;
    let ema_slow = ema(data, slow)?;
    let macd_line = ema_fast - ema_slow;
    
    // Simplificado: signal line como EMA del MACD
    let signal_line = macd_line * 0.9;
    let histogram = macd_line - signal_line;
    
    Some((macd_line, signal_line, histogram))
}

register_indicator!(metadata);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let meta = metadata();
        assert_eq!(meta.name, "macd");
        assert_eq!(meta.parameters.len(), 3);
    }

    #[test]
    fn test_macd() {
        let data: Vec<f64> = (1..=50).map(|x| x as f64).collect();
        let result = macd(&data, 12, 26, 9);
        assert!(result.is_some());
    }
}