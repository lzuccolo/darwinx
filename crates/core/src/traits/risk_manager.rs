//! Trait para gestión de riesgo

use crate::{Position, Signal};

/// Trait para gestores de riesgo
pub trait RiskManager: Send + Sync {
    /// Determina si se puede abrir una posición
    fn can_open_position(&self, signal: &Signal, balance: f64) -> bool;

    /// Calcula el tamaño de la posición
    fn calculate_position_size(&self, signal: &Signal, balance: f64) -> f64;

    /// Determina si se debe cerrar una posición
    fn should_close_position(&self, position: &Position, current_price: f64) -> bool;
}