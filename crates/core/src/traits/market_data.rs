//! Trait para acceso a datos de mercado

/// Abstracción sobre datos de mercado
///
/// Permite que las estrategias accedan a datos históricos
/// sin importar si vienen de live trading o backtesting
pub trait MarketData: Send + Sync {
    /// Retorna los últimos N precios de cierre
    fn close(&self, lookback: usize) -> &[f64];

    /// Retorna los últimos N volúmenes
    fn volume(&self, lookback: usize) -> &[f64];

    /// Retorna los últimos N precios máximos
    fn high(&self, lookback: usize) -> &[f64];

    /// Retorna los últimos N precios mínimos
    fn low(&self, lookback: usize) -> &[f64];

    /// Retorna la cantidad total de datos disponibles
    fn len(&self) -> usize;

    /// Retorna true si no hay datos
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}