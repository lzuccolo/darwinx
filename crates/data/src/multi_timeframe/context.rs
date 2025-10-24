//! Multi-timeframe context for trading strategies

use darwinx_core::{Candle, TimeFrame};
use std::collections::HashMap;

/// Data for a specific timeframe
pub struct TimeframeData {
    pub timeframe: TimeFrame,
    pub candles: Vec<Candle>,
}

impl TimeframeData {
    pub fn new(timeframe: TimeFrame, candles: Vec<Candle>) -> Self {
        Self { timeframe, candles }
    }

    /// Returns the last N closing prices
    pub fn close(&self, lookback: usize) -> Vec<f64> {
        self.candles
            .iter()
            .rev()
            .take(lookback)
            .map(|c| c.close)
            .collect()
    }

    /// Returns the last N high prices
    pub fn high(&self, lookback: usize) -> Vec<f64> {
        self.candles
            .iter()
            .rev()
            .take(lookback)
            .map(|c| c.high)
            .collect()
    }

    /// Returns the last N low prices
    pub fn low(&self, lookback: usize) -> Vec<f64> {
        self.candles
            .iter()
            .rev()
            .take(lookback)
            .map(|c| c.low)
            .collect()
    }

    /// Returns the last N volumes
    pub fn volume(&self, lookback: usize) -> Vec<f64> {
        self.candles
            .iter()
            .rev()
            .take(lookback)
            .map(|c| c.volume)
            .collect()
    }

    /// Gets the candle at or before a specific timestamp
    pub fn candle_at_timestamp(&self, timestamp: i64) -> Option<&Candle> {
        self.candles
            .iter()
            .rev()
            .find(|c| c.timestamp <= timestamp)
    }

    /// Total number of candles
    pub fn len(&self) -> usize {
        self.candles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.candles.is_empty()
    }
}

/// Multi-timeframe context for trading strategies
///
/// Provides access to data from multiple timeframes simultaneously
pub struct MultiTimeframeContext {
    data: HashMap<TimeFrame, TimeframeData>,
    primary_timeframe: TimeFrame,
}

impl MultiTimeframeContext {
    /// Creates a new context with a primary timeframe
    pub fn new(primary_timeframe: TimeFrame) -> Self {
        Self {
            data: HashMap::new(),
            primary_timeframe,
        }
    }

    /// Adds data for a timeframe
    pub fn add_timeframe(&mut self, timeframe: TimeFrame, candles: Vec<Candle>) {
        self.data
            .insert(timeframe, TimeframeData::new(timeframe, candles));
    }

    /// Gets data for a specific timeframe
    pub fn get(&self, timeframe: &TimeFrame) -> Option<&TimeframeData> {
        self.data.get(timeframe)
    }

    /// Gets data for the primary timeframe
    pub fn primary(&self) -> Option<&TimeframeData> {
        self.data.get(&self.primary_timeframe)
    }

    /// Returns all available timeframes
    pub fn timeframes(&self) -> Vec<TimeFrame> {
        self.data.keys().copied().collect()
    }

    /// Synchronizes all timeframes to the same timestamp
    pub fn sync_to_timestamp(&mut self, timestamp: i64) {
        for data in self.data.values_mut() {
            data.candles.retain(|c| c.timestamp <= timestamp);
        }
    }

    /// Gets the current candle for each timeframe at a specific timestamp
    /// Returns a map of timeframe -> candle for use in strategy evaluation
    pub fn get_sync_data(&self, timestamp: i64) -> HashMap<TimeFrame, &Candle> {
        let mut sync_data = HashMap::new();
        
        for (timeframe, data) in &self.data {
            if let Some(candle) = data.candle_at_timestamp(timestamp) {
                sync_data.insert(*timeframe, candle);
            }
        }
        
        sync_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_candles(count: usize, start_price: f64) -> Vec<Candle> {
        (0..count)
            .map(|i| {
                Candle::new(
                    (i as i64) * 60000, // 1 minute intervals
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
        let data = TimeframeData::new(TimeFrame::M1, candles);

        assert_eq!(data.len(), 10);
        
        let closes = data.close(5);
        assert_eq!(closes.len(), 5);
        assert_eq!(closes[0], 109.5); // Last candle
    }

    #[test]
    fn test_multi_timeframe_context() {
        let mut context = MultiTimeframeContext::new(TimeFrame::M1);

        // Add M1 data
        let m1_candles = create_test_candles(100, 100.0);
        context.add_timeframe(TimeFrame::M1, m1_candles);

        // Add H1 data
        let h1_candles = create_test_candles(20, 100.0);
        context.add_timeframe(TimeFrame::H1, h1_candles);

        // Verify
        assert_eq!(context.timeframes().len(), 2);
        assert!(context.get(&TimeFrame::M1).is_some());
        assert!(context.get(&TimeFrame::H1).is_some());
        assert!(context.primary().is_some());
    }

    #[test]
    fn test_sync_data() {
        let mut context = MultiTimeframeContext::new(TimeFrame::M1);

        let candles = create_test_candles(10, 100.0);
        context.add_timeframe(TimeFrame::M1, candles);

        // Get sync data at timestamp 300000 (5 minutes)
        let sync_data = context.get_sync_data(300000);
        
        assert!(sync_data.contains_key(&TimeFrame::M1));
        let candle = sync_data[&TimeFrame::M1];
        assert!(candle.timestamp <= 300000);
    }

    #[test]
    fn test_sync_to_timestamp() {
        let mut context = MultiTimeframeContext::new(TimeFrame::M1);

        let candles = create_test_candles(10, 100.0);
        context.add_timeframe(TimeFrame::M1, candles);

        // Sync to timestamp 300000 (5 minutes)
        context.sync_to_timestamp(300000);

        let data = context.primary().unwrap();
        assert!(data.len() <= 6); // Only candles up to timestamp 300000
    }
}