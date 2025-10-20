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
