use crate::register_indicator;

/// Metadata del indicador Stochastic
pub fn metadata() -> crate::metadata::IndicatorMetadata {
    use crate::metadata::*;
    
    IndicatorMetadata::new("stochastic")
        .category(IndicatorCategory::Momentum)
        .input_type(InputType::CandleSeries)
        .lookback(1)
        .parameter(ParameterDef::period("period", 5.0, 50.0, 14.0))
        .description("Stochastic Oscillator")
}

pub fn stochastic(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Option<f64> {
    if high.len() < period || low.len() < period || close.len() < period {
        return None;
    }
    
    let highest = high.iter().rev().take(period).fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let lowest = low.iter().rev().take(period).fold(f64::INFINITY, |a, &b| a.min(b));
    let current = close[close.len() - 1];
    
    if highest == lowest {
        return Some(50.0);
    }
    
    Some(((current - lowest) / (highest - lowest)) * 100.0)
}

register_indicator!(metadata);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let meta = metadata();
        assert_eq!(meta.name, "stochastic");
        assert_eq!(meta.parameters.len(), 1);
    }

    #[test]
    fn test_stochastic() {
        let high = vec![10.0, 11.0, 12.0, 11.5, 13.0];
        let low = vec![9.0, 9.5, 10.0, 10.5, 11.0];
        let close = vec![9.5, 10.5, 11.0, 11.0, 12.0];
        
        let result = stochastic(&high, &low, &close, 5);
        assert!(result.is_some());
    }
}