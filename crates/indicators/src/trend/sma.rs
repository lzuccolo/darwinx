/// Metadata del indicador SMA
pub fn metadata() -> crate::metadata::IndicatorMetadata {
    use crate::metadata::*;
    
    IndicatorMetadata::new("sma")
        .category(IndicatorCategory::Trend)
        .input_type(InputType::PriceSeries)
        .lookback(1)
        .parameter(ParameterDef::period("period", 2.0, 200.0, 20.0))
        .description("Simple Moving Average")
}

/// Calcula la media móvil simple
///
/// # Arguments
/// * `data` - Slice de precios
/// * `period` - Número de períodos
///
/// # Returns
/// * `Some(f64)` si hay suficientes datos
/// * `None` si no hay suficientes datos
pub fn sma(data: &[f64], period: usize) -> Option<f64> {
    if data.len() < period {
        return None;
    }
    let sum: f64 = data.iter().rev().take(period).sum();
    Some(sum / period as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let meta = metadata();
        assert_eq!(meta.name, "sma");
        assert_eq!(meta.parameters.len(), 1);
        assert_eq!(meta.parameters[0].name, "period");
    }

    #[test]
    fn test_sma() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(sma(&data, 5), Some(3.0));
        assert_eq!(sma(&data, 3), Some(4.0)); // (3+4+5)/3
    }

    #[test]
    fn test_sma_insufficient_data() {
        let data = vec![1.0, 2.0];
        assert_eq!(sma(&data, 5), None);
    }
}