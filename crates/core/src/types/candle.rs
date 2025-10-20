use serde::{Deserialize, Serialize};

/// Vela de precio (candlestick)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    /// Timestamp en milisegundos (Unix epoch)
    pub timestamp: i64,

    /// Precio de apertura
    pub open: f64,

    /// Precio máximo
    pub high: f64,

    /// Precio mínimo
    pub low: f64,

    /// Precio de cierre
    pub close: f64,

    /// Volumen negociado
    pub volume: f64,
}

impl Candle {
    /// Crea una nueva vela
    pub fn new(timestamp: i64, open: f64, high: f64, low: f64, close: f64, volume: f64) -> Self {
        Self {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
        }
    }

    /// Retorna el precio típico (high + low + close) / 3
    pub fn typical_price(&self) -> f64 {
        (self.high + self.low + self.close) / 3.0
    }

    /// Retorna true si la vela es alcista (cierre > apertura)
    pub fn is_bullish(&self) -> bool {
        self.close > self.open
    }

    /// Retorna true si la vela es bajista (cierre < apertura)
    pub fn is_bearish(&self) -> bool {
        self.close < self.open
    }

    /// Retorna el rango de la vela (high - low)
    pub fn range(&self) -> f64 {
        self.high - self.low
    }

    /// Retorna el cuerpo de la vela (|close - open|)
    pub fn body(&self) -> f64 {
        (self.close - self.open).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candle_creation() {
        let candle = Candle::new(1000, 100.0, 105.0, 95.0, 102.0, 1000.0);
        assert_eq!(candle.timestamp, 1000);
        assert_eq!(candle.open, 100.0);
        assert_eq!(candle.close, 102.0);
    }

    #[test]
    fn test_typical_price() {
        let candle = Candle::new(1000, 100.0, 110.0, 90.0, 105.0, 1000.0);
        let expected = (110.0 + 90.0 + 105.0) / 3.0;
        assert_eq!(candle.typical_price(), expected);
    }

    #[test]
    fn test_bullish_bearish() {
        let bullish = Candle::new(1000, 100.0, 105.0, 95.0, 102.0, 1000.0);
        assert!(bullish.is_bullish());
        assert!(!bullish.is_bearish());

        let bearish = Candle::new(1000, 100.0, 105.0, 95.0, 98.0, 1000.0);
        assert!(!bearish.is_bullish());
        assert!(bearish.is_bearish());
    }

    #[test]
    fn test_range_and_body() {
        let candle = Candle::new(1000, 100.0, 110.0, 90.0, 105.0, 1000.0);
        assert_eq!(candle.range(), 20.0);
        assert_eq!(candle.body(), 5.0);
    }
}