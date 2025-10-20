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
