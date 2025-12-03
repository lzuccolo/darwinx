//! # DarwinX Strategy Generator
//!
//! Generador de estrategias de trading usando algoritmos genéticos

pub mod ast;
pub mod generator;
pub mod constraints;

// Re-exports
pub use ast::nodes::{StrategyAST, Condition, IndicatorType};
pub use generator::random::RandomGenerator;
pub use generator::genetic::{GeneticGenerator, GeneticConfig};
pub use constraints::StrategyConstraints;