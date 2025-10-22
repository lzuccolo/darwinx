use crate::register_indicator;
use crate::trend::sma;

/// Metadata del indicador Bollinger Bands
pub fn metadata() -> crate::metadata::IndicatorMetadata {
    use crate::metadata::*;
    
    IndicatorMetadata::new("bollinger_bands")
        .category(IndicatorCategory::Volatility)
        .input_type(InputType::PriceSeries)
        .lookback(1)
        .parameter(ParameterDef::period("period", 5.0, 100.0, 20.0))
        .parameter(ParameterDef::multiplier("std_dev", 1.0, 5.0, 2.0))
        .description("Bollinger Bands")
}

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

register_indicator!(metadata);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let meta = metadata();
        assert_eq!(meta.name, "bollinger_bands");
        assert_eq!(meta.parameters.len(), 2);
    }

    #[test]
    fn test_bollinger_bands() {
        let data: Vec<f64> = (1..=30).map(|x| x as f64).collect();
        let result = bollinger_bands(&data, 20, 2.0);
        assert!(result.is_some());
        
        let (lower, middle, upper) = result.unwrap();
        assert!(lower < middle);
        assert!(middle < upper);
    }
}