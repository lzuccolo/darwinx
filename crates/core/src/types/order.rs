use serde::{Deserialize, Serialize};

/// Lado de la orden
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    /// Orden de compra
    Buy,
    /// Orden de venta
    Sell,
}

impl OrderSide {
    /// Convierte a string para APIs
    pub fn as_str(&self) -> &str {
        match self {
            OrderSide::Buy => "BUY",
            OrderSide::Sell => "SELL",
        }
    }
}

/// Orden ejecutada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// ID de la orden
    pub id: String,

    /// Símbolo del activo
    pub symbol: String,

    /// Lado de la orden
    pub side: OrderSide,

    /// Precio de ejecución
    pub price: f64,

    /// Cantidad ejecutada
    pub quantity: f64,

    /// Timestamp de ejecución
    pub timestamp: i64,
}

impl Order {
    /// Crea una nueva orden
    pub fn new(
        id: String,
        symbol: String,
        side: OrderSide,
        price: f64,
        quantity: f64,
        timestamp: i64,
    ) -> Self {
        Self {
            id,
            symbol,
            side,
            price,
            quantity,
            timestamp,
        }
    }

    /// Retorna el valor total de la orden
    pub fn total_value(&self) -> f64 {
        self.price * self.quantity
    }

    /// Retorna true si es una orden de compra
    pub fn is_buy(&self) -> bool {
        matches!(self.side, OrderSide::Buy)
    }

    /// Retorna true si es una orden de venta
    pub fn is_sell(&self) -> bool {
        matches!(self.side, OrderSide::Sell)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_creation() {
        let order = Order::new(
            "12345".to_string(),
            "BTCUSDT".to_string(),
            OrderSide::Buy,
            50000.0,
            0.1,
            1000,
        );

        assert_eq!(order.id, "12345");
        assert_eq!(order.symbol, "BTCUSDT");
        assert!(order.is_buy());
        assert!(!order.is_sell());
    }

    #[test]
    fn test_order_total_value() {
        let order = Order::new(
            "12345".to_string(),
            "BTCUSDT".to_string(),
            OrderSide::Buy,
            50000.0,
            0.1,
            1000,
        );

        assert_eq!(order.total_value(), 5000.0);
    }

    #[test]
    fn test_order_side_as_str() {
        assert_eq!(OrderSide::Buy.as_str(), "BUY");
        assert_eq!(OrderSide::Sell.as_str(), "SELL");
    }
}