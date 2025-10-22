use crate::register_indicator;

/// Metadata del indicador ATR
pub fn metadata() -> crate::metadata::IndicatorMetadata {
    use crate::metadata::*;
    
    IndicatorMetadata::new("atr")
        .category(IndicatorCategory::Volatility)
        .input_type(InputType::CandleSeries)
        .lookback(1)
        .parameter(ParameterDef::period("period", 2.0, 100.0, 14.0))
        .description("Average True Range")
}

pub fn atr(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Option<f64> {
    if high.len() < period + 1 || low.len() < period + 1 || close.len() < period + 1 {
        return None;
    }
    
    let mut tr_sum = 0.0;
    
    for i in 1..=period {
        let idx = high.len() - i;
        let h_l = high[idx] - low[idx];
        let h_c = (high[idx] - close[idx - 1]).abs();
        let l_c = (low[idx] - close[idx - 1]).abs();
        tr_sum += h_l.max(h_c).max(l_c);
    }
    
    Some(tr_sum / period as f64)
}

register_indicator!(metadata);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let meta = metadata();
        assert_eq!(meta.name, "atr");
        assert_eq!(meta.parameters.len(), 1);
    }

    #[test]
    fn test_atr() {
        let high = vec![10.0, 11.0, 12.0, 11.5, 13.0, 12.8, 14.0];
        let low = vec![9.0, 9.5, 10.0, 10.5, 11.0, 11.5, 12.0];
        let close = vec![9.5, 10.5, 11.0, 11.0, 12.0, 12.5, 13.0];
        
        let result = atr(&high, &low, &close, 5);
        assert!(result.is_some());
    }
}