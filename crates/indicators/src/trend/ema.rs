/// Metadata del indicador EMA
pub fn metadata() -> crate::metadata::IndicatorMetadata {
    use crate::metadata::*;

    IndicatorMetadata::new("ema")
        .category(IndicatorCategory::Trend)
        .input_type(InputType::PriceSeries)
        .lookback(1)
        .parameter(ParameterDef::period("period", 2.0, 200.0, 12.0))
        .description("Exponential Moving Average")
}

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
