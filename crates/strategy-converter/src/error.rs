//! Error handling para conversiones de estrategias

use thiserror::Error;

/// Errores de conversión de estrategias
#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Error de parsing: {0}")]
    ParseError(String),

    #[error("Error de formato: {0}")]
    FormatError(String),

    #[error("Error de validación: {0}")]
    ValidationError(String),

    #[error("Formato no soportado: {0}")]
    UnsupportedFormat(String),

    #[error("Error de sintaxis: {0}")]
    SyntaxError(String),

    #[error("Error de semántica: {0}")]
    SemanticError(String),

    #[error("Error interno: {0}")]
    InternalError(#[from] anyhow::Error),
}

