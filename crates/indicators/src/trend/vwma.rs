/// Metadata del indicador VWMA
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