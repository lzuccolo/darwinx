//! Abstract Syntax Tree para estrategias multi-timeframe
//!
//! Este módulo contiene la representación AST de las estrategias de trading
//! con soporte completo para multi-timeframe strategies.

pub mod nodes;
pub mod builder;
pub mod validator;

// Re-exports principales
pub use nodes::*;
pub use builder::{StrategyBuilder, ConditionBuilder};
pub use validator::{StrategyValidator, ValidationResult};
