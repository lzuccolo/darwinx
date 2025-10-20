/// Calcula la media móvil ponderada
pub fn wma(data: &[f64], period: usize) -> Option<f64> {
    if data.len() < period {
        return None;
    }
    
    let weights_sum = (period * (period + 1)) / 2;
    let mut weighted_sum = 0.0;
    
    for (i, &price) in data.iter().rev().take(period).enumerate() {
        weighted_sum += price * (period - i) as f64;
    }
    
    Some(weighted_sum / weights_sum as f64)
}
