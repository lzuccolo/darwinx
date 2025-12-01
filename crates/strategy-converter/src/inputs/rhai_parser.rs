//! Parser de scripts Rhai a AST
//!
//! Este módulo parsea scripts Rhai y los convierte a StrategyAST

use crate::error::ConversionError;
use darwinx_generator::StrategyAST;

/// Parsea un script Rhai y lo convierte a AST
///
/// # Example
/// ```rhai
/// strategy_timeframe("5m");
/// let rsi = indicator("rsi", [14], "current");
/// entry_rules("and", [rsi < 30.0]);
/// exit_rules("or", [rsi > 70.0]);
/// ```
pub fn parse_rhai(_script: &str) -> Result<StrategyAST, ConversionError> {
    // TODO: Implementar parser completo de Rhai
    // Por ahora, retornar error indicando que no está implementado
    
    Err(ConversionError::UnsupportedFormat(
        "Rhai parser not yet implemented. This is a placeholder.".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rhai_parser_placeholder() {
        let script = r#"
            strategy_timeframe("5m");
            let rsi = indicator("rsi", [14], "current");
            entry_rules("and", [rsi < 30.0]);
        "#;

        let result = parse_rhai(script);
        assert!(result.is_err()); // Por ahora retorna error
    }
}

