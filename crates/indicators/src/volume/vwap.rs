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