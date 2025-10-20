pub fn atr(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Option<f64> {
    if high.len() < period + 1 || low.len() < period + 1 || close.len() < period + 1 {
        return None;
    }

    let mut tr_sum = 0.0;
    for i in 1..=period {
        let idx = high.len() - i;
        let h_l = high[idx] - low[idx];
        let h_c = (high[idx] - close[idx - 1]).abs();
        let l_c = (low[idx] - close[idx - 1]).abs();
        tr_sum += h_l.max(h_c).max(l_c);
    }

    Some(tr_sum / period as f64)
}
