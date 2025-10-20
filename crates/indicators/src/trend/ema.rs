pub fn ema(data: &[f64], period: usize) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    let k = 2.0 / (period as f64 + 1.0);
    let mut ema_val = data[0];
    
    for &price in data.iter().skip(1) {
        ema_val = price * k + ema_val * (1.0 - k);
    }
    Some(ema_val)
}