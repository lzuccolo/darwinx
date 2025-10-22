use crate::register_indicator;

/// Metadata del indicador OBV
pub fn metadata() -> crate::metadata::IndicatorMetadata {
    use crate::metadata::*;
    
    IndicatorMetadata::new("obv")
        .category(IndicatorCategory::Volume)
        .input_type(InputType::MultiSeries)
        .lookback(1)
        .description("On Balance Volume")
}

pub fn obv(close: &[f64], volume: &[f64]) -> Option<Vec<f64>> {
    if close.len() != volume.len() || close.len() < 2 {
        return None;
    }
    
    let mut obv_values = vec![volume[0]];
    
    for i in 1..close.len() {
        let prev_obv = obv_values[i - 1];
        if close[i] > close[i - 1] {
            obv_values.push(prev_obv + volume[i]);
        } else if close[i] < close[i - 1] {
            obv_values.push(prev_obv - volume[i]);
        } else {
            obv_values.push(prev_obv);
        }
    }
    
    Some(obv_values)
}

register_indicator!(metadata);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let meta = metadata();
        assert_eq!(meta.name, "obv");
        assert_eq!(meta.parameters.len(), 0);
    }

    #[test]
    fn test_obv() {
        let close = vec![10.0, 11.0, 10.5, 12.0, 11.5];
        let volume = vec![100.0, 150.0, 120.0, 200.0, 180.0];
        
        let result = obv(&close, &volume);
        assert!(result.is_some());
        
        let obv_values = result.unwrap();
        assert_eq!(obv_values.len(), 5);
        assert_eq!(obv_values[0], 100.0);
    }

    #[test]
    fn test_obv_mismatched_lengths() {
        let close = vec![10.0, 11.0];
        let volume = vec![100.0];
        
        assert!(obv(&close, &volume).is_none());
    }
}