//! Constraints y límites para estrategias multi-timeframe
//!
//! Este módulo contiene el sistema de constraints actualizado con soporte
//! para semantic constraints y anti-correlación.

pub mod strategy;
pub mod semantic;

// Re-exports principales
pub use strategy::StrategyConstraints;
pub use semantic::SemanticConstraints;
