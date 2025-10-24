//! Simple timeframe synchronization for multi-timeframe strategies

use darwinx_core::{Candle, TimeFrame};
use std::collections::HashMap;

/// Simple timeframe synchronizer
pub struct TimeframeSynchronizer;

impl TimeframeSynchronizer {
    /// Synchronizes multiple timeframes to a specific timestamp
    /// Returns the last candle from each timeframe at or before the timestamp
    pub fn sync_timeframes(
        data: &HashMap<TimeFrame, Vec<Candle>>,
        timestamp: i64,
    ) -> HashMap<TimeFrame, Candle> {
        let mut synced = HashMap::new();

        for (timeframe, candles) in data {
            if let Some(candle) = Self::find_candle_at_timestamp(candles, timestamp) {
                synced.insert(*timeframe, candle.clone());
            }
        }

        synced
    }

    /// Finds the candle at or immediately before a timestamp
    fn find_candle_at_timestamp(candles: &[Candle], timestamp: i64) -> Option<&Candle> {
        candles
            .iter()
            .rev()
            .find(|c| c.timestamp <= timestamp)
    }

    /// Forward-fills missing data using the last available candle
    pub fn forward_fill(
        base_candle: &Candle,
        target_timestamp: i64,
    ) -> Candle {
        Candle::new(
            target_timestamp,
            base_candle.close, // Use last close as new open
            base_candle.close, // No price movement
            base_candle.close,
            base_candle.close,
            0.0, // Zero volume for forward-filled data
        )
    }

    /// Checks if timeframes are compatible for synchronization
    pub fn are_timeframes_compatible(tf1: TimeFrame, tf2: TimeFrame) -> bool {
        // All timeframes are compatible for basic sync
        // This could be extended with more sophisticated rules
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_candles(start_ts: i64, count: usize, interval_ms: i64) -> Vec<Candle> {
        (0..count)
            .map(|i| {
                Candle::new(
                    start_ts + (i as i64 * interval_ms),
                    100.0 + i as f64,
                    101.0 + i as f64,
                    99.0 + i as f64,
                    100.5 + i as f64,
                    1000.0,
                )
            })
            .collect()
    }

    #[test]
    fn test_sync_timeframes() {
        let mut data = HashMap::new();
        
        // M1 data (1-minute intervals)
        let m1_candles = create_test_candles(0, 10, 60_000);
        data.insert(TimeFrame::M1, m1_candles);
        
        // M5 data (5-minute intervals)
        let m5_candles = create_test_candles(0, 3, 300_000);
        data.insert(TimeFrame::M5, m5_candles);

        // Sync at 8 minutes
        let synced = TimeframeSynchronizer::sync_timeframes(&data, 480_000);

        assert_eq!(synced.len(), 2);
        assert!(synced.contains_key(&TimeFrame::M1));
        assert!(synced.contains_key(&TimeFrame::M5));
    }

    #[test]
    fn test_forward_fill() {
        let base_candle = Candle::new(100_000, 100.0, 101.0, 99.0, 100.5, 1000.0);
        let filled = TimeframeSynchronizer::forward_fill(&base_candle, 200_000);

        assert_eq!(filled.timestamp, 200_000);
        assert_eq!(filled.open, 100.5); // Last close becomes new open
        assert_eq!(filled.close, 100.5);
        assert_eq!(filled.volume, 0.0); // Zero volume for filled data
    }

    #[test]
    fn test_find_candle_at_timestamp() {
        let candles = create_test_candles(0, 5, 60_000);
        
        // Find exact match
        let candle = TimeframeSynchronizer::find_candle_at_timestamp(&candles, 120_000);
        assert!(candle.is_some());
        assert_eq!(candle.unwrap().timestamp, 120_000);
        
        // Find previous candle when no exact match
        let candle = TimeframeSynchronizer::find_candle_at_timestamp(&candles, 150_000);
        assert!(candle.is_some());
        assert_eq!(candle.unwrap().timestamp, 120_000); // Previous candle
    }
}