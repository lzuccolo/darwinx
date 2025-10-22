use crate::register_indicator;
use crate::trend::ema;
use crate::volatility::atr;

/// Metadata del indicador Keltner Channels
pub fn metadata() -> crate::metadata::IndicatorMetadata {
    use crate::metadata::*;
    
    IndicatorMetadata::new("keltner_channels")
        .category(IndicatorCategory::Volatility)
        .input_type(InputType::CandleSeries)
        .lookback(1)
        .parameter(ParameterDef::period("period", 5.0, 100.0, 20.0))
        .parameter(ParameterDef::multiplier("multiplier", 1.0, 5.0, 2.0))
        .description("Keltner Channels")
}

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

register_indicator!(metadata);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let meta = metadata();
        assert_eq!(meta.name, "keltner_channels");
        assert_eq!(meta.parameters.len(), 2);
    }

    #[test]
    fn test_keltner_channels() {
        let high: Vec<f64> = (10..40).map(|x| x as f64).collect();
        let low: Vec<f64> = (8..38).map(|x| x as f64).collect();
        let close: Vec<f64> = (9..39).map(|x| x as f64).collect();
        
        let result = keltner_channels(&high, &low, &close, 20, 2.0);
        assert!(result.is_some());
        
        let (lower, middle, upper) = result.unwrap();
        assert!(lower < middle);
        assert!(middle < upper);
    }
}