//! Loaders para diferentes formatos de datos

pub mod csv;
pub mod parquet;

#[cfg(test)]
mod integration_tests;

pub use csv::CsvLoader;
pub use parquet::ParquetLoader;

use crate::multi_timeframe::MultiTimeframeContext;
use darwinx_core::TimeFrame;
use std::collections::HashMap;

/// Helper functions para cargar datos multi-timeframe
pub struct MultiTimeframeLoader;

impl MultiTimeframeLoader {
    /// Carga múltiples timeframes desde archivos CSV y crea un contexto multi-timeframe
    /// 
    /// # Arguments
    /// * `paths` - Mapa de timeframe -> path del archivo CSV
    /// * `primary_timeframe` - Timeframe principal
    /// 
    /// # Example
    /// ```rust
    /// use darwinx_data::{MultiTimeframeLoader, CsvLoader};
    /// use darwinx_core::TimeFrame;
    /// use std::collections::HashMap;
    /// 
    /// let mut paths = HashMap::new();
    /// paths.insert(TimeFrame::M5, "data/m5.csv");
    /// paths.insert(TimeFrame::H1, "data/h1.csv");
    /// 
    /// let context = MultiTimeframeLoader::load_multi_csv(&paths, TimeFrame::M5)?;
    /// ```
    pub fn load_multi_csv(
        paths: &HashMap<TimeFrame, &str>,
        primary_timeframe: TimeFrame,
    ) -> anyhow::Result<MultiTimeframeContext> {
        let mut context = MultiTimeframeContext::new(primary_timeframe);

        for (timeframe, path) in paths {
            let candles = CsvLoader::load(path)?;
            context.add_timeframe(*timeframe, candles);
        }

        Ok(context)
    }

    /// Carga múltiples timeframes desde archivos Parquet y crea un contexto multi-timeframe
    /// 
    /// # Arguments
    /// * `paths` - Mapa de timeframe -> path del archivo Parquet
    /// * `primary_timeframe` - Timeframe principal
    pub fn load_multi_parquet(
        paths: &HashMap<TimeFrame, &str>,
        primary_timeframe: TimeFrame,
    ) -> anyhow::Result<MultiTimeframeContext> {
        let mut context = MultiTimeframeContext::new(primary_timeframe);

        for (timeframe, path) in paths {
            let candles = ParquetLoader::load(path)?;
            context.add_timeframe(*timeframe, candles);
        }

        Ok(context)
    }

    /// Carga un solo timeframe y crea un contexto multi-timeframe simple
    /// 
    /// Útil para casos donde solo se necesita un timeframe pero se quiere usar
    /// la misma API que multi-timeframe.
    pub fn load_single_csv(
        path: &str,
        timeframe: TimeFrame,
    ) -> anyhow::Result<MultiTimeframeContext> {
        let candles = CsvLoader::load(path)?;
        let mut context = MultiTimeframeContext::new(timeframe);
        context.add_timeframe(timeframe, candles);
        Ok(context)
    }

    /// Carga un solo timeframe desde Parquet
    pub fn load_single_parquet(
        path: &str,
        timeframe: TimeFrame,
    ) -> anyhow::Result<MultiTimeframeContext> {
        let candles = ParquetLoader::load(path)?;
        let mut context = MultiTimeframeContext::new(timeframe);
        context.add_timeframe(timeframe, candles);
        Ok(context)
    }
}