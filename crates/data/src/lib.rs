//! # DarwinX Data
//!
//! Manejo de datos de mercado: carga, procesamiento y multi-timeframe.

pub mod loader;
pub mod multi_timeframe;

// Re-exports
pub use loader::{CsvLoader, ParquetLoader};
pub use multi_timeframe::MultiTimeFrameContext;