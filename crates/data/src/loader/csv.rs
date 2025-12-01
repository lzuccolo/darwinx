//! Loader para archivos CSV

use darwinx_core::Candle;
use polars::prelude::*;
use std::sync::Arc;

/// Loader para archivos CSV
pub struct CsvLoader;

impl CsvLoader {
    pub fn load(path: &str) -> anyhow::Result<Vec<Candle>> {
        // Especificar tipos de datos para evitar inferencia incorrecta
        let schema = Schema::from_iter([
            Field::new("timestamp".into(), DataType::Int64),
            Field::new("open".into(), DataType::Float64),
            Field::new("high".into(), DataType::Float64),
            Field::new("low".into(), DataType::Float64),
            Field::new("close".into(), DataType::Float64),
            Field::new("volume".into(), DataType::Float64),
        ]);

        let df = CsvReadOptions::default()
            .with_has_header(true)
            .with_schema(Some(Arc::new(schema)))
            .try_into_reader_with_file_path(Some(path.into()))?
            .finish()?;

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
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_csv_loader() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            "timestamp,open,high,low,close,volume\n\
             1609459200000,29000.0,29500.0,28800.0,29200.0,1000.0\n\
             1609545600000,29200.0,29800.0,29100.0,29500.0,1200.0"
        )
        .unwrap();

        let candles = CsvLoader::load(file.path().to_str().unwrap()).unwrap();

        assert_eq!(candles.len(), 2);
        assert_eq!(candles[0].close, 29200.0);
    }
}