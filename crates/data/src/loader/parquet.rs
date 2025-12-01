//! Loader para archivos Parquet

use darwinx_core::Candle;
use polars::prelude::{ParquetReader, SerReader};
use std::fs::File;

/// Loader para archivos Parquet
pub struct ParquetLoader;

impl ParquetLoader {
    pub fn load(path: &str) -> anyhow::Result<Vec<Candle>> {
        // Read Parquet file (Polars 0.51 - following trading-engine pattern)
        let mut file = File::open(path)
            .map_err(|e| anyhow::anyhow!("Failed to open Parquet file: {}", e))?;
        let df = ParquetReader::new(&mut file)
            .finish()
            .map_err(|e| anyhow::anyhow!("Failed to read Parquet file: {}", e))?;

        let timestamps = df.column("timestamp")?.i64()?;
        let opens = df.column("open")?.f64()?;
        let highs = df.column("high")?.f64()?;
        let lows = df.column("low")?.f64()?;
        let closes = df.column("close")?.f64()?;
        let volumes = df.column("volume")?.f64()?;

        let mut candles = Vec::new();
        for i in 0..df.height() {
            let timestamp = timestamps.get(i).ok_or_else(|| {
                anyhow::anyhow!("Missing timestamp at index {}", i)
            })?;
            let open = opens.get(i).ok_or_else(|| {
                anyhow::anyhow!("Missing open at index {}", i)
            })?;
            let high = highs.get(i).ok_or_else(|| {
                anyhow::anyhow!("Missing high at index {}", i)
            })?;
            let low = lows.get(i).ok_or_else(|| {
                anyhow::anyhow!("Missing low at index {}", i)
            })?;
            let close = closes.get(i).ok_or_else(|| {
                anyhow::anyhow!("Missing close at index {}", i)
            })?;
            let volume = volumes.get(i).ok_or_else(|| {
                anyhow::anyhow!("Missing volume at index {}", i)
            })?;

            candles.push(Candle::new(timestamp, open, high, low, close, volume));
        }

        Ok(candles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::prelude::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parquet_loader() {
        use polars::prelude::*;

        // Crear DataFrame temporal
        let timestamps = Series::new("timestamp".into(), &[1609459200000i64, 1609545600000i64]);
        let opens = Series::new("open".into(), &[29000.0, 29200.0]);
        let highs = Series::new("high".into(), &[29500.0, 29800.0]);
        let lows = Series::new("low".into(), &[28800.0, 29100.0]);
        let closes = Series::new("close".into(), &[29200.0, 29500.0]);
        let volumes = Series::new("volume".into(), &[1000.0, 1200.0]);

        let df = DataFrame::new(vec![
            timestamps.into(),
            opens.into(),
            highs.into(),
            lows.into(),
            closes.into(),
            volumes.into(),
        ]).unwrap();

        // Guardar como Parquet
        let file = NamedTempFile::new().unwrap();
        {
            let mut file_handle = std::fs::File::create(file.path()).unwrap();
            ParquetWriter::new(&mut file_handle)
                .finish(&mut df.clone())
                .unwrap();
        }

        // Cargar
        let candles = ParquetLoader::load(file.path().to_str().unwrap()).unwrap();

        assert_eq!(candles.len(), 2);
        assert_eq!(candles[0].close, 29200.0);
    }
}