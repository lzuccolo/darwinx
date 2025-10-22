use crate::register_indicator;

/// Metadata del indicador MFI
pub fn metadata() -> crate::metadata::IndicatorMetadata {
    use crate::metadata::*;
    
    IndicatorMetadata::new("mfi")
        .category(IndicatorCategory::Volume)
        .input_type(InputType::MultiSeries)
        .lookback(1)
        .parameter(ParameterDef::period("period", 2.0, 100.0, 14.0))
        .description("Money Flow Index")
}

pub fn mfi(high: &[f64], low: &[f64], close: &[f64], volume: &[f64], period: usize) -> Option<f64> {
    if high.len() < period + 1 || low.len() < period + 1 || close.len() < period + 1 || volume.len() < period + 1 {
        return None;
    }
    
    let mut positive_flow = 0.0;
    let mut negative_flow = 0.0;
    
    for i in 1..=period {
        let idx = close.len() - i;
        let typical = (high[idx] + low[idx] + close[idx]) / 3.0;
        let typical_prev = (high[idx - 1] + low[idx - 1] + close[idx - 1]) / 3.0;
        let money_flow = typical * volume[idx];
        
        if typical > typical_prev {
            positive_flow += money_flow;
        } else {
            negative_flow += money_flow;
        }
    }
    
    if negative_flow == 0.0 {
        return Some(100.0);
    }
    
    let money_ratio = positive_flow / negative_flow;
    Some(100.0 - (100.0 / (1.0 + money_ratio)))
}

register_indicator!(metadata);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let meta = metadata();
        assert_eq!(meta.name, "mfi");
        assert_eq!(meta.parameters.len(), 1);
    }

    #[test]
    fn test_mfi() {
        let high: Vec<f64> = (10..25).map(|x| x as f64).collect();
        let low: Vec<f64> = (8..23).map(|x| x as f64).collect();
        let close: Vec<f64> = (9..24).map(|x| x as f64).collect();
        let volume: Vec<f64> = (100..115).map(|x| x as f64 * 1000.0).collect();
        
        let result = mfi(&high, &low, &close, &volume, 14);
        assert!(result.is_some());
    }
}