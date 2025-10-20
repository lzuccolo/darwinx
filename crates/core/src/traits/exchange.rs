//! Trait para exchanges

use crate::{Candle, Order, OrderSide, Position};
use async_trait::async_trait;

/// Trait para exchanges de criptomonedas
#[async_trait]
pub trait Exchange: Send + Sync {
    /// Obtiene el balance disponible
    async fn get_balance(&self) -> Result<f64, String>;

    /// Coloca una orden en el exchange
    async fn place_order(
        &self,
        side: OrderSide,
        symbol: &str,
        quantity: f64,
    ) -> Result<Order, String>;

    /// Obtiene las posiciones abiertas
    async fn get_open_positions(&self) -> Result<Vec<Position>, String>;

    /// Cierra una posición
    async fn close_position(&self, position: &Position) -> Result<Order, String>;

    /// Stream de velas en tiempo real
    async fn stream_candles(
        &self,
        symbol: &str,
    ) -> Result<tokio::sync::mpsc::Receiver<Candle>, String>;
}