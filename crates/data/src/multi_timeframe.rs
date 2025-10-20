//! Contexto multi-timeframe para estrategias

use darwinx_core::{Candle, TimeFrame};
use std::collections::HashMap;

/// Datos de un timeframe específico
pub struct TimeFrameData {
    pub timeframe: TimeFrame,
    pub candles: Vec<Candle>,
}

impl TimeFrameData {
    pub fn new(timeframe: TimeFrame, candles: Vec<Candle>) -> Self {
        Self { timeframe, candles }
    }

    /// Retorna los últimos N precios de cierre
    pub fn close(&self, lookback: usize) -> Vec<f64> {
        self.candles
            .iter()
            .rev()
            .take(lookback)
            .map(|c| c.close)
            .collect()
    }

    /// Retorna los últimos N precios máximos
    pub fn high(&self, lookback: usize) -> Vec<f64> {
        self.candles
            .iter()
            .rev()
            .take(lookback)
            .map(|c| c.high)
            .collect()
    }

    /// Retorna los últimos N precios mínimos
    pub fn low(&self, lookback: usize) -> Vec<f64> {
        self.candles
            .iter()
            .rev()
            .take(lookback)
            .map(|c| c.low)
            .collect()
    }

    /// Retorna los últimos N volúmenes
    pub fn volume(&self, lookback: usize) -> Vec<f64> {
        self.candles
            .iter()
            .rev()
            .take(lookback)
            .map(|c| c.volume)
            .collect()
    }

    /// Cantidad total de velas
    pub fn len(&self) -> usize {
        self.candles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.candles.is_empty()
    }
}

/// Contexto multi-timeframe
///
/// Permite acceder a datos de múltiples timeframes simultáneamente
pub struct MultiTimeFrameContext {
    data: HashMap<TimeFrame, TimeFrameData>,
    primary_timeframe: TimeFrame,
}

impl MultiTimeFrameContext {
    /// Crea un nuevo contexto con un timeframe primario
    pub fn new(primary_timeframe: TimeFrame) -> Self {
        Self {
            data: HashMap::new(),
            primary_timeframe,
        }
    }

    /// Agrega datos de un timeframe
    pub fn add_timeframe(&mut self, timeframe: TimeFrame, candles: Vec<Candle>) {
        self.data
            .insert(timeframe, TimeFrameData::new(timeframe, candles));
    }

    /// Obtiene datos de un timeframe específico
    pub fn get(&self, timeframe: &TimeFrame) -> Option<&TimeFrameData> {
        self.data.get(timeframe)
    }

    /// Obtiene datos del timeframe primario
    pub fn primary(&self) -> Option<&TimeFrameData> {
        self.data.get(&self.primary_timeframe)
    }

    /// Retorna todos los timeframes disponibles
    pub fn timeframes(&self) -> Vec<TimeFrame> {
        self.data.keys().copied().collect()
    }

    /// Sincroniza todos los timeframes al mismo timestamp
    pub fn sync_to_timestamp(&mut self, timestamp: i64) {
        for data in self.data.values_mut() {
            data.candles.retain(|c| c.timestamp <= timestamp);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_candles(count: usize, start_price: f64) -> Vec<Candle> {
        (0..count)
            .map(|i| {
                Candle::new(
                    (i as i64) * 60000,
                    start_price + i as f64,
                    start_price + i as f64 + 1.0,
                    start_price + i as f64 - 1.0,
                    start_price + i as f64 + 0.5,
                    1000.0,
                )
            })
            .collect()
    }

    #[test]
    fn test_timeframe_data() {
        let candles = create_test_candles(10, 100.0);
        let data = TimeFrameData::new(TimeFrame::M1, candles);

        assert_eq!(data.len(), 10);
        
        let closes = data.close(5);
        assert_eq!(closes.len(), 5);
        assert_eq!(closes[0], 109.5); // Último
    }

    #[test]
    fn test_multi_timeframe_context() {
        let mut context = MultiTimeFrameContext::new(TimeFrame::M1);

        // Agregar M1
        let m1_candles = create_test_candles(100, 100.0);
        context.add_timeframe(TimeFrame::M1, m1_candles);

        // Agregar H1
        let h1_candles = create_test_candles(20, 100.0);
        context.add_timeframe(TimeFrame::H1, h1_candles);

        // Verificar
        assert_eq!(context.timeframes().len(), 2);
        assert!(context.get(&TimeFrame::M1).is_some());
        assert!(context.get(&TimeFrame::H1).is_some());
        assert!(context.primary().is_some());
    }

    #[test]
    fn test_sync_to_timestamp() {
        let mut context = MultiTimeFrameContext::new(TimeFrame::M1);

        let candles = create_test_candles(10, 100.0);
        context.add_timeframe(TimeFrame::M1, candles);

        // Sincronizar al timestamp 300000 (5 minutos)
        context.sync_to_timestamp(300000);

        let data = context.primary().unwrap();
        assert!(data.len() <= 6); // Solo velas hasta timestamp 300000
    }
}