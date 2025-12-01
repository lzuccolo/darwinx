//! Trait principal para conversión de estrategias

use crate::error::ConversionError;
use crate::formats::StrategyFormat;
use darwinx_generator::StrategyAST;

/// Trait principal para conversión entre formatos de estrategias
pub trait StrategyConverter: Send + Sync {
    /// Convierte desde un formato a AST
    fn from_format(
        &self,
        input: &str,
        format: StrategyFormat,
    ) -> Result<StrategyAST, ConversionError>;

    /// Convierte desde AST a un formato
    fn to_format(
        &self,
        ast: &StrategyAST,
        format: StrategyFormat,
    ) -> Result<String, ConversionError>;

    /// Convierte entre dos formatos (usando AST como intermediario)
    fn convert(
        &self,
        input: &str,
        from: StrategyFormat,
        to: StrategyFormat,
    ) -> Result<String, ConversionError> {
        let ast = self.from_format(input, from)?;
        self.to_format(&ast, to)
    }
}

/// Implementación por defecto del converter
pub struct DefaultStrategyConverter;

impl DefaultStrategyConverter {
    /// Crea un nuevo converter
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultStrategyConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl StrategyConverter for DefaultStrategyConverter {
    fn from_format(
        &self,
        input: &str,
        format: StrategyFormat,
    ) -> Result<StrategyAST, ConversionError> {
        match format {
            StrategyFormat::AST => {
                // AST es JSON serializado
                serde_json::from_str(input)
                    .map_err(|e| ConversionError::ParseError(format!("Failed to parse AST: {}", e)))
            }
            StrategyFormat::Rhai => {
                // Usar parser de Rhai
                crate::inputs::rhai_parser::parse_rhai(input)
            }
            StrategyFormat::Rust => {
                Err(ConversionError::UnsupportedFormat(
                    "Rust to AST conversion not yet implemented".to_string(),
                ))
            }
            StrategyFormat::Python => {
                Err(ConversionError::UnsupportedFormat(
                    "Python to AST conversion not yet implemented".to_string(),
                ))
            }
            StrategyFormat::Freqtrade => {
                Err(ConversionError::UnsupportedFormat(
                    "Freqtrade to AST conversion not yet implemented".to_string(),
                ))
            }
        }
    }

    fn to_format(
        &self,
        ast: &StrategyAST,
        format: StrategyFormat,
    ) -> Result<String, ConversionError> {
        match format {
            StrategyFormat::AST => {
                // AST a JSON
                serde_json::to_string_pretty(ast)
                    .map_err(|e| ConversionError::FormatError(format!("Failed to serialize AST: {}", e)))
            }
            StrategyFormat::Rhai => {
                // AST a Rhai script
                crate::outputs::rhai_generator::generate_rhai(ast)
            }
            StrategyFormat::Rust => {
                // AST a Rust trait Strategy
                crate::outputs::rust_generator::generate_rust(ast)
            }
            StrategyFormat::Python => {
                // AST a Python
                crate::outputs::python_generator::generate_python(ast)
            }
            StrategyFormat::Freqtrade => {
                // AST a Freqtrade
                crate::outputs::freqtrade_generator::generate_freqtrade(ast)
            }
        }
    }
}

