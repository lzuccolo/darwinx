//! Tests de integración para loaders con multi-timeframe

#[cfg(test)]
mod tests {
    use crate::MultiTimeframeLoader;
    use darwinx_core::TimeFrame;
    use std::collections::HashMap;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_csv(timeframe: TimeFrame, count: usize) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "timestamp,open,high,low,close,volume").unwrap();

        let base_timestamp = 1609459200000i64; // 2021-01-01
        let interval = timeframe.to_millis();

        for i in 0..count {
            let timestamp = base_timestamp + (i as i64 * interval);
            let price = 29000.0 + (i as f64 * 10.0);
            writeln!(
                file,
                "{},{},{},{},{},{}",
                timestamp,
                price,
                price + 100.0,
                price - 100.0,
                price + 50.0,
                1000.0 + (i as f64 * 10.0)
            )
            .unwrap();
        }

        file
    }

    #[test]
    fn test_load_single_csv() {
        let file = create_test_csv(TimeFrame::M5, 10);
        let context = MultiTimeframeLoader::load_single_csv(
            file.path().to_str().unwrap(),
            TimeFrame::M5,
        )
        .unwrap();

        assert_eq!(context.timeframes().len(), 1);
        assert!(context.timeframes().contains(&TimeFrame::M5));

        let primary_data = context.primary().unwrap();
        assert_eq!(primary_data.len(), 10);
    }

    #[test]
    fn test_load_multi_csv() {
        let m5_file = create_test_csv(TimeFrame::M5, 100);
        let h1_file = create_test_csv(TimeFrame::H1, 20);

        let mut paths = HashMap::new();
        paths.insert(TimeFrame::M5, m5_file.path().to_str().unwrap());
        paths.insert(TimeFrame::H1, h1_file.path().to_str().unwrap());

        let context = MultiTimeframeLoader::load_multi_csv(&paths, TimeFrame::M5).unwrap();

        assert_eq!(context.timeframes().len(), 2);
        assert!(context.timeframes().contains(&TimeFrame::M5));
        assert!(context.timeframes().contains(&TimeFrame::H1));

        let m5_data = context.get(&TimeFrame::M5).unwrap();
        assert_eq!(m5_data.len(), 100);

        let h1_data = context.get(&TimeFrame::H1).unwrap();
        assert_eq!(h1_data.len(), 20);
    }

    #[test]
    fn test_load_single_parquet() {
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

        // Cargar con multi-timeframe loader
        let context = MultiTimeframeLoader::load_single_parquet(
            file.path().to_str().unwrap(),
            TimeFrame::H1,
        )
        .unwrap();

        assert_eq!(context.timeframes().len(), 1);
        let data = context.primary().unwrap();
        assert_eq!(data.len(), 2);
    }

    #[test]
    fn test_multi_timeframe_sync() {
        let m5_file = create_test_csv(TimeFrame::M5, 100);
        let h1_file = create_test_csv(TimeFrame::H1, 20);

        let mut paths = HashMap::new();
        paths.insert(TimeFrame::M5, m5_file.path().to_str().unwrap());
        paths.insert(TimeFrame::H1, h1_file.path().to_str().unwrap());

        let context = MultiTimeframeLoader::load_multi_csv(&paths, TimeFrame::M5).unwrap();

        // Test sincronización
        let base_timestamp = 1609459200000i64;
        let sync_data = context.get_sync_data(base_timestamp + (60 * 60 * 1000)); // 1 hora después

        assert!(sync_data.contains_key(&TimeFrame::M5));
        assert!(sync_data.contains_key(&TimeFrame::H1));

        // Verificar que las velas están sincronizadas
        let m5_candle = sync_data.get(&TimeFrame::M5).unwrap();
        let h1_candle = sync_data.get(&TimeFrame::H1).unwrap();

        assert!(m5_candle.timestamp <= h1_candle.timestamp + TimeFrame::H1.to_millis());
    }
}

