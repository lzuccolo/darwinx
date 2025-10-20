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
