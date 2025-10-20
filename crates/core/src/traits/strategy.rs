//! Trait para estrategias de trading

use crate::{MarketData, Signal};

/// Trait que deben implementar todas las estrategias de trading
pub trait Strategy: Send + Sync {
    /// Nombre de la estrategia
    fn name(&self) -> &str;

    /// Evalúa la estrategia y retorna una señal
    fn evaluate(&mut self, data: &dyn MarketData) -> Signal;

    /// Número mínimo de períodos necesarios para evaluar
    fn min_periods(&self) -> usize;

    /// Reinicia el estado interno de la estrategia
    fn reset(&mut self);

    /// Descripción opcional de la estrategia
    fn description(&self) -> Option<&str> {
        None
    }
}
