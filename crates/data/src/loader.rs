//! Loaders para diferentes formatos de datos

pub mod csv;
pub mod parquet;

pub use csv::CsvLoader;
pub use parquet::ParquetLoader;