//! Simple timeframe alignment utilities

use darwinx_core::{Candle, TimeFrame};

/// Simple timeframe aligner for strategy evaluation
pub struct TimeframeAligner;

impl TimeframeAligner {
    /// Aligns a timestamp to the nearest timeframe boundary
    /// 
    /// For example:
    /// - Timestamp 90000 (1.5 min) aligned to M1 -> 60000 (1 min)
    /// - Timestamp 450000 (7.5 min) aligned to M5 -> 300000 (5 min)
    pub fn align_timestamp(timestamp: i64, timeframe: TimeFrame) -> i64 {
        let interval_ms = timeframe.to_millis();
        (timestamp / interval_ms) * interval_ms
    }

    /// Creates a list of aligned timestamps for a time range
    pub fn create_aligned_timestamps(
        start: i64,
        end: i64,
        timeframe: TimeFrame,
    ) -> Vec<i64> {
        let interval_ms = timeframe.to_millis();
        let aligned_start = Self::align_timestamp(start, timeframe);
        
        let mut timestamps = Vec::new();
        let mut current = aligned_start;
        
        while current <= end {
            timestamps.push(current);
            current += interval_ms;
        }
        
        timestamps
    }

    /// Checks if a candle is aligned to its timeframe
    pub fn is_aligned(candle: &Candle, timeframe: TimeFrame) -> bool {
        let aligned = Self::align_timestamp(candle.timestamp, timeframe);
        candle.timestamp == aligned
    }

    /// Aligns candles to timeframe boundaries, filtering out misaligned ones
    pub fn filter_aligned_candles(candles: &[Candle], timeframe: TimeFrame) -> Vec<Candle> {
        candles
            .iter()
            .filter(|c| Self::is_aligned(c, timeframe))
            .cloned()
            .collect()
    }

    /// Gets the next timeframe boundary after a given timestamp
    pub fn next_boundary(timestamp: i64, timeframe: TimeFrame) -> i64 {
        let interval_ms = timeframe.to_millis();
        let aligned = Self::align_timestamp(timestamp, timeframe);
        
        if aligned == timestamp {
            aligned + interval_ms
        } else {
            aligned + interval_ms
        }
    }

    /// Gets the previous timeframe boundary before a given timestamp
    pub fn previous_boundary(timestamp: i64, timeframe: TimeFrame) -> i64 {
        let interval_ms = timeframe.to_millis();
        let aligned = Self::align_timestamp(timestamp, timeframe);
        
        if aligned == timestamp {
            aligned - interval_ms
        } else {
            aligned
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_timestamp() {
        // Test M1 alignment (60,000 ms intervals)
        assert_eq!(TimeframeAligner::align_timestamp(0, TimeFrame::M1), 0);
        assert_eq!(TimeframeAligner::align_timestamp(30_000, TimeFrame::M1), 0);
        assert_eq!(TimeframeAligner::align_timestamp(60_000, TimeFrame::M1), 60_000);
        assert_eq!(TimeframeAligner::align_timestamp(90_000, TimeFrame::M1), 60_000);
        
        // Test M5 alignment (300,000 ms intervals)
        assert_eq!(TimeframeAligner::align_timestamp(0, TimeFrame::M5), 0);
        assert_eq!(TimeframeAligner::align_timestamp(150_000, TimeFrame::M5), 0);
        assert_eq!(TimeframeAligner::align_timestamp(300_000, TimeFrame::M5), 300_000);
        assert_eq!(TimeframeAligner::align_timestamp(450_000, TimeFrame::M5), 300_000);
    }

    #[test]
    fn test_create_aligned_timestamps() {
        let timestamps = TimeframeAligner::create_aligned_timestamps(
            30_000,  // 0.5 min
            250_000, // 4.16 min
            TimeFrame::M1
        );
        
        // Should create timestamps: 0, 60000, 120000, 180000, 240000
        assert_eq!(timestamps.len(), 5);
        assert_eq!(timestamps[0], 0);
        assert_eq!(timestamps[1], 60_000);
        assert_eq!(timestamps[4], 240_000);
    }

    #[test]
    fn test_is_aligned() {
        let aligned_candle = Candle::new(60_000, 100.0, 101.0, 99.0, 100.5, 1000.0);
        let misaligned_candle = Candle::new(90_000, 100.0, 101.0, 99.0, 100.5, 1000.0);
        
        assert!(TimeframeAligner::is_aligned(&aligned_candle, TimeFrame::M1));
        assert!(!TimeframeAligner::is_aligned(&misaligned_candle, TimeFrame::M1));
    }

    #[test]
    fn test_next_boundary() {
        assert_eq!(TimeframeAligner::next_boundary(0, TimeFrame::M1), 60_000);
        assert_eq!(TimeframeAligner::next_boundary(30_000, TimeFrame::M1), 60_000);
        assert_eq!(TimeframeAligner::next_boundary(60_000, TimeFrame::M1), 120_000);
    }

    #[test]
    fn test_previous_boundary() {
        assert_eq!(TimeframeAligner::previous_boundary(60_000, TimeFrame::M1), 0);
        assert_eq!(TimeframeAligner::previous_boundary(90_000, TimeFrame::M1), 60_000);
        assert_eq!(TimeframeAligner::previous_boundary(120_000, TimeFrame::M1), 60_000);
    }

    #[test]
    fn test_filter_aligned_candles() {
        let candles = vec![
            Candle::new(0, 100.0, 101.0, 99.0, 100.5, 1000.0),      // Aligned
            Candle::new(30_000, 100.0, 101.0, 99.0, 100.5, 1000.0), // Not aligned
            Candle::new(60_000, 100.0, 101.0, 99.0, 100.5, 1000.0), // Aligned
            Candle::new(90_000, 100.0, 101.0, 99.0, 100.5, 1000.0), // Not aligned
        ];
        
        let aligned = TimeframeAligner::filter_aligned_candles(&candles, TimeFrame::M1);
        assert_eq!(aligned.len(), 2);
        assert_eq!(aligned[0].timestamp, 0);
        assert_eq!(aligned[1].timestamp, 60_000);
    }
}