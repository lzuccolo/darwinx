use crate::register_indicator;

pub fn metadata() -> crate::metadata::IndicatorMetadata {
    use crate::metadata::*;
    
    IndicatorMetadata::new("wma")
        .category(IndicatorCategory::Trend)
        .input_type(InputType::PriceSeries)
        .lookback(1)
        .parameter(ParameterDef::period("period", 2.0, 200.0, 20.0))
        .description("Weighted Moving Average")
}

/// Calcula la media móvil ponderada
pub fn wma(data: &[f64], period: usize) -> Option<f64> {
    if data.len() < period {
        return None;
    }
    
    let weights_sum = (period * (period + 1)) / 2;
    let mut weighted_sum = 0.0;
    
    for (i, &price) in data.iter().rev().take(period).enumerate() {
        weighted_sum += price * (period - i) as f64;
    }
    
    Some(weighted_sum / weights_sum as f64)
}

register_indicator!(metadata);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let meta = metadata();
        assert_eq!(meta.name, "wma");
        assert_eq!(meta.parameters.len(), 1);
        assert_eq!(meta.parameters[0].name, "period");
    }

    #[test]
    fn test_wma() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = wma(&data, 5);
        assert!(result.is_some());
        // WMA = (1*1 + 2*2 + 3*3 + 4*4 + 5*5) / (1+2+3+4+5)
        // WMA = (1 + 4 + 9 + 16 + 25) / 15 = 55/15 = 3.666...
        let wma_val = result.unwrap();
        assert!((wma_val - 3.666).abs() < 0.01);
    }

    #[test]
    fn test_wma_insufficient_data() {
        let data = vec![1.0, 2.0];
        assert!(wma(&data, 5).is_none());
    }
}