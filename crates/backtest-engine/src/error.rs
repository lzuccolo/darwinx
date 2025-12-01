//! Error handling para el Backtest Engine

use thiserror::Error;

/// Errores del Backtest Engine
#[derive(Debug, Error)]
pub enum BacktestError {
    #[error("Error de datos: {0}")]
    DataError(#[from] anyhow::Error),

    #[error("Error de estrategia: {0}")]
    StrategyError(String),

    #[error("Error de configuración: {0}")]
    ConfigError(String),

    #[error("Error de métricas: {0}")]
    MetricsError(String),

    #[error("Error de ejecución: {0}")]
    ExecutionError(String),

    #[error("Balance insuficiente: requerido {required}, disponible {available}")]
    InsufficientBalance { required: f64, available: f64 },

    #[error("Posición no encontrada: {0}")]
    PositionNotFound(String),
}

