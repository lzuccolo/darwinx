use crate::register_indicator;

/// Metadata del indicador ROC
pub fn metadata() -> crate::metadata::IndicatorMetadata {
    use crate::metadata::*;
    
    IndicatorMetadata::new("roc")
        .category(IndicatorCategory::Momentum)
        .input_type(InputType::PriceSeries)
        .lookback(1)
        .parameter(ParameterDef::period("period", 1.0, 100.0, 12.0))
        .description("Rate of Change")
}

pub fn roc(data: &[f64], period: usize) -> Option<f64> {
    if data.len() < period + 1 {
        return None;
    }
    
    let current = data[data.len() - 1];
    let previous = data[data.len() - period - 1];
    
    if previous == 0.0 {
        return None;
    }
    
    Some(((current - previous) / previous) * 100.0)
}

register_indicator!(metadata);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let meta = metadata();
        assert_eq!(meta.name, "roc");
        assert_eq!(meta.parameters.len(), 1);
    }

    #[test]
    fn test_roc() {
        let data = vec![100.0, 105.0, 110.0, 108.0, 112.0];
        let result = roc(&data, 3);
        assert!(result.is_some());
    }
}