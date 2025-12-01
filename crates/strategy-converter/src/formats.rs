//! Formatos de estrategias soportados

use serde::{Deserialize, Serialize};

/// Formatos de estrategias que el converter puede manejar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrategyFormat {
    /// Abstract Syntax Tree (formato interno)
    AST,
    /// Rhai script (DSL para estrategias)
    Rhai,
    /// Rust trait Strategy (código ejecutable)
    Rust,
    /// Python script
    Python,
    /// Freqtrade strategy format
    Freqtrade,
}

impl StrategyFormat {
    /// Retorna todos los formatos disponibles
    pub fn all() -> Vec<Self> {
        vec![
            Self::AST,
            Self::Rhai,
            Self::Rust,
            Self::Python,
            Self::Freqtrade,
        ]
    }

    /// Retorna el nombre del formato como string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AST => "ast",
            Self::Rhai => "rhai",
            Self::Rust => "rust",
            Self::Python => "python",
            Self::Freqtrade => "freqtrade",
        }
    }

    /// Parsea desde string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "ast" => Some(Self::AST),
            "rhai" => Some(Self::Rhai),
            "rust" => Some(Self::Rust),
            "python" | "py" => Some(Self::Python),
            "freqtrade" => Some(Self::Freqtrade),
            _ => None,
        }
    }
}

impl std::fmt::Display for StrategyFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

