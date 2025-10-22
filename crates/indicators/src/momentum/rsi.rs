use crate::register_indicator;

/// Metadata del indicador RSI
pub fn metadata() -> crate::metadata::IndicatorMetadata {
    use crate::metadata::*;
    
    IndicatorMetadata::new("rsi")
        .category(IndicatorCategory::Momentum)
        .input_type(InputType::PriceSeries)
        .lookback(1)
        .parameter(ParameterDef::period("period", 2.0, 100.0, 14.0))
        .description("Relative Strength Index")
}

pub fn rsi(data: &[f64], period: usize) -> Option<f64> {
    if data.len() < period + 1 {
        return None;
    }
    
    let mut gains = 0.0;
    let mut losses = 0.0;
    
    for i in 1..=period {
        let change = data[data.len() - i] - data[data.len() - i - 1];
        if change > 0.0 {
            gains += change;
        } else {
            losses -= change;
        }
    }
    
    let avg_gain = gains / period as f64;
    let avg_loss = losses / period as f64;
    
    if avg_loss == 0.0 {
        return Some(100.0);
    }
    
    let rs = avg_gain / avg_loss;
    Some(100.0 - (100.0 / (1.0 + rs)))
}

register_indicator!(metadata);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let meta = metadata();
        assert_eq!(meta.name, "rsi");
        assert_eq!(meta.parameters.len(), 1);
    }

    #[test]
    fn test_rsi() {
        let data = vec![44.0, 44.5, 45.0, 43.5, 44.0, 45.0, 46.0, 45.5];
        let result = rsi(&data, 5);
        assert!(result.is_some());
    }
}