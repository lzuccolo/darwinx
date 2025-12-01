//! Strategy Converter Hub - Hub central de conversión de estrategias
//!
//! Este crate proporciona conversión bidireccional entre diferentes formatos:
//! - Rhai scripts ↔ AST
//! - AST → Rust (trait Strategy)
//! - AST → Python
//! - AST → Freqtrade
//!
//! # Principios de Diseño
//!
//! - **Hub Central**: Punto único de conversión entre formatos
//! - **Bidireccional**: Conversión en ambas direcciones cuando es posible
//! - **Extensible**: Fácil agregar nuevos formatos
//! - **Type-safe**: Validación de conversiones

pub mod error;
pub mod formats;
pub mod converter;
pub mod inputs;
pub mod outputs;

// Re-exports
pub use error::ConversionError;
pub use formats::StrategyFormat;
pub use converter::StrategyConverter;

