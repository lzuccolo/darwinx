pub fn stochastic(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Option<f64> {
    if high.len() < period || low.len() < period || close.len() < period {
        return None;
    }
    
    let highest = high.iter().rev().take(period).fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let lowest = low.iter().rev().take(period).fold(f64::INFINITY, |a, &b| a.min(b));
    let current = close[close.len() - 1];
    
    if highest == lowest {
        return Some(50.0);
    }
    
    Some(((current - lowest) / (highest - lowest)) * 100.0)
}