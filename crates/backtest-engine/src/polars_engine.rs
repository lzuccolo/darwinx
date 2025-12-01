//! Motor Polars vectorizado para backtest masivo
//!
//! Este módulo implementa el motor de backtest usando Polars para
//! procesamiento vectorizado y paralelo.

pub mod vectorized;

pub use vectorized::PolarsBacktestEngine;

