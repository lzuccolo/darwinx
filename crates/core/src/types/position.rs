use serde::{Deserialize, Serialize};

/// Lado de la posición
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionSide {
    /// Posición larga (comprada)
    Long,
    /// Posición corta (vendida)
    Short,
}

/// Posición abierta en el mercado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// Símbolo del activo
    pub symbol: String,

    /// Lado de la posición
    pub side: PositionSide,

    /// Precio de entrada
    pub entry_price: f64,

    /// Cantidad
    pub quantity: f64,

    /// Timestamp de apertura
    pub timestamp: i64,
}

impl Position {
    /// Crea una nueva posición
    pub fn new(
        symbol: String,
        side: PositionSide,
        entry_price: f64,
        quantity: f64,
        timestamp: i64,
    ) -> Self {
        Self {
            symbol,
            side,
            entry_price,
            quantity,
            timestamp,
        }
    }

    /// Calcula el PnL actual de la posición
    pub fn calculate_pnl(&self, current_price: f64) -> f64 {
        match self.side {
            PositionSide::Long => (current_price - self.entry_price) * self.quantity,
            PositionSide::Short => (self.entry_price - current_price) * self.quantity,
        }
    }

    /// Calcula el PnL porcentual
    pub fn calculate_pnl_percent(&self, current_price: f64) -> f64 {
        match self.side {
            PositionSide::Long => ((current_price - self.entry_price) / self.entry_price) * 100.0,
            PositionSide::Short => {
                ((self.entry_price - current_price) / self.entry_price) * 100.0
            }
        }
    }

    /// Retorna el valor total de la posición al precio actual
    pub fn value_at(&self, price: f64) -> f64 {
        price * self.quantity
    }

    /// Retorna el valor total de entrada
    pub fn entry_value(&self) -> f64 {
        self.entry_price * self.quantity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long_position_pnl() {
        let position = Position::new("BTCUSDT".to_string(), PositionSide::Long, 100.0, 1.0, 0);

        // Precio sube: ganancia
        assert_eq!(position.calculate_pnl(110.0), 10.0);
        assert_eq!(position.calculate_pnl_percent(110.0), 10.0);

        // Precio baja: pérdida
        assert_eq!(position.calculate_pnl(90.0), -10.0);
        assert_eq!(position.calculate_pnl_percent(90.0), -10.0);
    }

    #[test]
    fn test_short_position_pnl() {
        let position = Position::new("BTCUSDT".to_string(), PositionSide::Short, 100.0, 1.0, 0);

        // Precio baja: ganancia
        assert_eq!(position.calculate_pnl(90.0), 10.0);
        assert_eq!(position.calculate_pnl_percent(90.0), 10.0);

        // Precio sube: pérdida
        assert_eq!(position.calculate_pnl(110.0), -10.0);
        assert_eq!(position.calculate_pnl_percent(110.0), -10.0);
    }

    #[test]
    fn test_position_values() {
        let position = Position::new("BTCUSDT".to_string(), PositionSide::Long, 100.0, 2.0, 0);

        assert_eq!(position.entry_value(), 200.0);
        assert_eq!(position.value_at(110.0), 220.0);
    }
}