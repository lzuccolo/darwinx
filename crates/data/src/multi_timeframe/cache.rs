//! Simple data cache for multi-timeframe operations

use darwinx_core::{Candle, TimeFrame};
use std::collections::HashMap;

/// Simple cache for multi-timeframe data
pub struct MultiTimeframeDataCache {
    data: HashMap<TimeFrame, Vec<Candle>>,
    max_candles_per_timeframe: usize,
}

impl MultiTimeframeDataCache {
    /// Creates a new cache with default settings
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            max_candles_per_timeframe: 10_000, // Reasonable default
        }
    }

    /// Creates a cache with custom capacity
    pub fn with_capacity(max_candles_per_timeframe: usize) -> Self {
        Self {
            data: HashMap::new(),
            max_candles_per_timeframe,
        }
    }

    /// Stores candles for a timeframe, maintaining size limits
    pub fn store(&mut self, timeframe: TimeFrame, mut candles: Vec<Candle>) {
        // Sort by timestamp to ensure order
        candles.sort_by_key(|c| c.timestamp);
        
        // Limit size to prevent memory issues
        if candles.len() > self.max_candles_per_timeframe {
            let start_idx = candles.len() - self.max_candles_per_timeframe;
            candles = candles[start_idx..].to_vec();
        }
        
        self.data.insert(timeframe, candles);
    }

    /// Retrieves candles for a timeframe
    pub fn get(&self, timeframe: &TimeFrame) -> Option<&Vec<Candle>> {
        self.data.get(timeframe)
    }

    /// Updates candles for a timeframe (adds new candles)
    pub fn update(&mut self, timeframe: TimeFrame, new_candles: Vec<Candle>) {
        if let Some(existing) = self.data.get_mut(&timeframe) {
            existing.extend(new_candles);
            existing.sort_by_key(|c| c.timestamp);
            existing.dedup_by_key(|c| c.timestamp); // Remove duplicates
            
            // Maintain size limit
            if existing.len() > self.max_candles_per_timeframe {
                let start_idx = existing.len() - self.max_candles_per_timeframe;
                *existing = existing[start_idx..].to_vec();
            }
        } else {
            self.store(timeframe, new_candles);
        }
    }

    /// Clears all cached data
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Removes data for a specific timeframe
    pub fn remove(&mut self, timeframe: &TimeFrame) {
        self.data.remove(timeframe);
    }

    /// Returns all available timeframes
    pub fn timeframes(&self) -> Vec<TimeFrame> {
        self.data.keys().copied().collect()
    }

    /// Returns the number of candles for a timeframe
    pub fn len(&self, timeframe: &TimeFrame) -> usize {
        self.data.get(timeframe).map_or(0, |v| v.len())
    }

    /// Checks if cache is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Gets memory usage statistics
    pub fn memory_usage(&self) -> CacheStats {
        let total_candles: usize = self.data.values().map(|v| v.len()).sum();
        let timeframe_count = self.data.len();
        
        CacheStats {
            total_candles,
            timeframe_count,
            estimated_memory_mb: (total_candles * std::mem::size_of::<Candle>()) / (1024 * 1024),
        }
    }
}

impl Default for MultiTimeframeDataCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_candles: usize,
    pub timeframe_count: usize,
    pub estimated_memory_mb: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_candles(start_ts: i64, count: usize) -> Vec<Candle> {
        (0..count)
            .map(|i| {
                Candle::new(
                    start_ts + (i as i64 * 60_000),
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
    fn test_cache_store_and_get() {
        let mut cache = MultiTimeframeDataCache::new();
        let candles = create_test_candles(0, 100);
        
        cache.store(TimeFrame::M1, candles);
        
        assert_eq!(cache.len(&TimeFrame::M1), 100);
        assert!(cache.get(&TimeFrame::M1).is_some());
    }

    #[test]
    fn test_cache_update() {
        let mut cache = MultiTimeframeDataCache::new();
        
        // Store initial data
        let initial_candles = create_test_candles(0, 50);
        cache.store(TimeFrame::M1, initial_candles);
        
        // Update with new data
        let new_candles = create_test_candles(50 * 60_000, 50);
        cache.update(TimeFrame::M1, new_candles);
        
        assert_eq!(cache.len(&TimeFrame::M1), 100);
    }

    #[test]
    fn test_cache_size_limit() {
        let mut cache = MultiTimeframeDataCache::with_capacity(50);
        let candles = create_test_candles(0, 100);
        
        cache.store(TimeFrame::M1, candles);
        
        // Should be limited to 50 candles
        assert_eq!(cache.len(&TimeFrame::M1), 50);
    }

    #[test]
    fn test_cache_stats() {
        let mut cache = MultiTimeframeDataCache::new();
        
        cache.store(TimeFrame::M1, create_test_candles(0, 100));
        cache.store(TimeFrame::M5, create_test_candles(0, 50));
        
        let stats = cache.memory_usage();
        assert_eq!(stats.total_candles, 150);
        assert_eq!(stats.timeframe_count, 2);
    }
}