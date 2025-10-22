use crate::register_indicator;

pub fn metadata() -> crate::metadata::IndicatorMetadata {
    use crate::metadata::*;
    
    IndicatorMetadata::new("vwma")
        .category(IndicatorCategory::Trend)
        .input_type(InputType::MultiSeries)
        .lookback(1)
        .parameter(ParameterDef::period("period", 2.0, 200.0, 20.0))
        .description("Volume Weighted Moving Average")
}

/// Calcula la media móvil ponderada por volumen
pub fn vwma(prices: &[f64], volumes: &[f64], period: usize) -> Option<f64> {
    if prices.len() < period || volumes.len() < period {
        return None;
    }
    
    let mut sum_pv = 0.0;
    let mut sum_v = 0.0;
    
    for i in 0..period {
        let idx = prices.len() - period + i;
        sum_pv += prices[idx] * volumes[idx];
        sum_v += volumes[idx];
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
        assert_eq!(meta.name, "vwma");
        assert_eq!(meta.parameters.len(), 1);
        assert_eq!(meta.parameters[0].name, "period");
    }

    #[test]
    fn test_vwma() {
        let prices = vec![10.0, 11.0, 12.0, 11.5, 13.0];
        let volumes = vec![100.0, 150.0, 120.0, 200.0, 180.0];
        
        let result = vwma(&prices, &volumes, 5);
        assert!(result.is_some());
    }

    #[test]
    fn test_vwma_insufficient_data() {
        let prices = vec![10.0, 11.0];
        let volumes = vec![100.0, 150.0];
        
        assert!(vwma(&prices, &volumes, 5).is_none());
    }

    #[test]
    fn test_vwma_zero_volume() {
        let prices = vec![10.0, 11.0, 12.0];
        let volumes = vec![0.0, 0.0, 0.0];
        
        assert!(vwma(&prices, &volumes, 3).is_none());
    }
}