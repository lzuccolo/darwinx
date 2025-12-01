//! Generador de código Rust (trait Strategy) desde AST

use crate::error::ConversionError;
use darwinx_generator::StrategyAST;

/// Genera código Rust que implementa el trait Strategy
pub fn generate_rust(ast: &StrategyAST) -> Result<String, ConversionError> {
    // TODO: Implementar generador completo
    // Por ahora, retornar un stub
    
    Ok(format!(
        "// Generated Rust code from AST: {}\n// TODO: Implement full generation",
        ast.name
    ))
}

