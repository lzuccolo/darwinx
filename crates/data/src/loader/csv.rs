//! Loader para archivos CSV

use darwinx_core::Candle;
use polars::prelude::*;

/// Loader para archivos CSV
pub struct CsvLoader;

impl CsvLoader {
    pub fn load(path: &str) -> anyhow::Result<Vec<Candle>> {
        let df = CsvReadOptions::default()
            .with_has_header(true)
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