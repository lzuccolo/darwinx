//! Loader para archivos Parquet

use darwinx_core::Candle;
use polars::prelude::*;

/// Loader para archivos Parquet
pub struct ParquetLoader;

impl ParquetLoader {
    pub fn load(path: &str) -> anyhow::Result<Vec<Candle>> {
        let df = LazyFrame::scan_parquet(path, Default::default())?
            .collect()?;

        let timestamps = df.column("timestamp")?.i64()?;
        let opens = df.column("open")?.f64()?;
        let highs = df.column("high")?.f64()?;
        let lows = df.column("low")?.f64()?;
        let closes = df.column("close")?.f64()?;
        let volumes = df.column("volume")?.f64()?;

        let mut candles = Vec::new();
        for i in 0..df.height() {
            candles.push(Candle::new(
                timestamps.get(i).unwrap(),
                opens.get(i).unwrap(),
                highs.get(i).unwrap(),
                lows.get(i).unwrap(),
                closes.get(i).unwrap(),
                volumes.get(i).unwrap(),
            ));
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
        // Crear DataFrame temporal
        let timestamps = Series::new("timestamp", &[1609459200000i64, 1609545600000i64]);
        let opens = Series::new("open", &[29000.0, 29200.0]);
        let highs = Series::new("high", &[29500.0, 29800.0]);
        let lows = Series::new("low", &[28800.0, 29100.0]);
        let closes = Series::new("close", &[29200.0, 29500.0]);
        let volumes = Series::new("volume", &[1000.0, 1200.0]);

        let df = DataFrame::new(vec![timestamps, opens, highs, lows, closes, volumes]).unwrap();

        // Guardar como Parquet
        let file = NamedTempFile::new().unwrap();
        let mut writer = ParquetWriter::new(std::fs::File::create(file.path()).unwrap());
        writer.finish(&mut df.clone()).unwrap();

        // Cargar
        let candles = ParquetLoader::load(file.path().to_str().unwrap()).unwrap();

        assert_eq!(candles.len(), 2);
        assert_eq!(candles[0].close, 29200.0);
    }
}