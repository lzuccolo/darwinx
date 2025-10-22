use crate::register_indicator;

/// Metadata del indicador VWAP
pub fn metadata() -> crate::metadata::IndicatorMetadata {
    use crate::metadata::*;
    
    IndicatorMetadata::new("vwap")
        .category(IndicatorCategory::Volume)
        .input_type(InputType::MultiSeries)
        .lookback(1)
        .description("Volume Weighted Average Price")
}

pub fn vwap(high: &[f64], low: &[f64], close: &[f64], volume: &[f64]) -> Option<f64> {
    if high.is_empty() || low.is_empty() || close.is_empty() || volume.is_empty() {
        return None;
    }
    
    let len = high.len().min(low.len()).min(close.len()).min(volume.len());
    
    let mut sum_pv = 0.0;
    let mut sum_v = 0.0;
    
    for i in 0..len {
        let typical = (high[i] + low[i] + close[i]) / 3.0;
        sum_pv += typical * volume[i];
        sum_v += volume[i];
    }
    
    if sum_v == 0.0 {
        return None;
    }
    
    Some(sum_pv / sum_v)
}

register_indicator!(metadata);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let meta = metadata();
        assert_eq!(meta.name, "vwap");
        assert_eq!(meta.parameters.len(), 0);
    }

    #[test]
    fn test_vwap() {
        let high = vec![11.0, 12.0, 13.0, 12.5, 14.0];
        let low = vec![9.0, 10.0, 11.0, 11.5, 12.0];
        let close = vec![10.0, 11.0, 12.0, 12.0, 13.0];
        let volume = vec![1000.0, 1500.0, 1200.0, 1300.0, 1100.0];
        
        let result = vwap(&high, &low, &close, &volume);
        assert!(result.is_some());
    }

    #[test]
    fn test_vwap_empty() {
        let empty: Vec<f64> = vec![];
        assert!(vwap(&empty, &empty, &empty, &empty).is_none());
    }
}