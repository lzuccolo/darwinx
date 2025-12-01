//! Generadores de estrategias multi-timeframe
//!
//! Este módulo contiene los generadores de estrategias actualizados con soporte
//! para multi-timeframe y semantic constraints.

pub mod random;
pub mod genetic;

// Re-exports principales
pub use random::RandomGenerator;
pub use genetic::GeneticGenerator;
