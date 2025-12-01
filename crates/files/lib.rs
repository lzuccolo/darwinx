//! # DarwinX Strategy Generator v2.1 - Multi-Timeframe Edition
//!
//! Generador de estrategias de trading multi-timeframe usando algoritmos genéticos
//! con constraints semánticos para evitar correlación entre indicadores.
//!
//! ## Nuevas características v2.1:
//! - ✨ **Multi-timeframe support**: Strategies can combine indicators from different timeframes
//! - ✨ **TimeframeCategory**: Current/Medium/High relative to primary timeframe  
//! - ✨ **Semantic Constraints**: Anti-correlation constraints based on real data
//! - ✨ **Enhanced Builder**: Multi-TF builder methods
//! - ✨ **Smart Validation**: Multi-TF validation with timeframe consistency checks
//!
//! ## Architecture:
//! ```text
//! strategy-generator/
//! ├── ast/           → AST nodes with multi-timeframe support
//! │   ├── nodes.rs   → StrategyAST, TimeframeCategory, IndicatorType
//! │   ├── builder.rs → Multi-TF builder methods
//! │   └── validator.rs → Multi-TF validation logic
//! ├── generator/     → Strategy generators with semantic constraints  
//! │   ├── random.rs  → Random generator with constraints
//! │   └── genetic.rs → Genetic algorithm with diversity fitness
//! └── constraints/   → Enhanced constraints system
//!     ├── strategy.rs → Basic strategy constraints
//!     └── semantic.rs → Semantic constraints (correlation, etc.)
//! ```

pub mod ast;
pub mod generator; 
pub mod constraints;

// Re-exports for easy access
pub use ast::nodes::{StrategyAST, TimeframeCategory, IndicatorType, Condition, LogicalOperator};
pub use ast::builder::{StrategyBuilder, ConditionBuilder};
pub use ast::validator::{StrategyValidator, ValidationResult};
pub use generator::random::RandomGenerator;
pub use generator::genetic::GeneticGenerator;
pub use constraints::strategy::StrategyConstraints;
pub use constraints::semantic::SemanticConstraints;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const EDITION: &str = "Multi-Timeframe v2.1";

#[cfg(test)]
mod integration_tests {
    use super::*;
    use darwinx_core::TimeFrame;

    #[test]
    fn test_multi_timeframe_strategy_creation() {
        let strategy = StrategyBuilder::new("Multi-TF Test".to_string(), TimeFrame::M5)
            .add_entry_condition_with_timeframe(
                ConditionBuilder::above("rsi", vec![14.0], 50.0),
                TimeframeCategory::Current
            )
            .add_entry_condition_with_timeframe(
                ConditionBuilder::above("ema", vec![200.0], 100.0),
                TimeframeCategory::Medium  // 15m timeframe
            )
            .build();

        assert_eq!(strategy.primary_timeframe, TimeFrame::M5);
        assert_eq!(strategy.entry_rules.conditions.len(), 2);
        
        // Verify timeframe categories
        assert_eq!(strategy.entry_rules.conditions[0].indicator.timeframe_category, TimeframeCategory::Current);
        assert_eq!(strategy.entry_rules.conditions[1].indicator.timeframe_category, TimeframeCategory::Medium);
    }

    #[test]
    fn test_semantic_constraints() {
        let semantic_constraints = SemanticConstraints::default();
        assert!(semantic_constraints.max_similarity_score > 0.0);
        assert!(semantic_constraints.max_similarity_score <= 1.0);
    }
}
