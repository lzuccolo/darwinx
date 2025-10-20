use serde::{Deserialize, Serialize};

/// Señal de trading generada por una estrategia
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Signal {
    /// Señal de compra
    Buy {
        /// Precio sugerido
        price: f64,
        /// Nivel de confianza (0.0 - 1.0)
        confidence: f64,
    },

    /// Señal de venta
    Sell {
        /// Precio sugerido
        price: f64,
        /// Nivel de confianza (0.0 - 1.0)
        confidence: f64,
    },

    /// Mantener posición actual
    Hold,
}

impl Signal {
    /// Retorna el precio de la señal, si aplica
    pub fn price(&self) -> Option<f64> {
        match self {
            Signal::Buy { price, .. } | Signal::Sell { price, .. } => Some(*price),
            Signal::Hold => None,
        }
    }

    /// Retorna la confianza de la señal, si aplica
    pub fn confidence(&self) -> Option<f64> {
        match self {
            Signal::Buy { confidence, .. } | Signal::Sell { confidence, .. } => Some(*confidence),
            Signal::Hold => None,
        }
    }

    /// Retorna true si es una señal de compra
    pub fn is_buy(&self) -> bool {
        matches!(self, Signal::Buy { .. })
    }

    /// Retorna true si es una señal de venta
    pub fn is_sell(&self) -> bool {
        matches!(self, Signal::Sell { .. })
    }

    /// Retorna true si es Hold
    pub fn is_hold(&self) -> bool {
        matches!(self, Signal::Hold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_buy() {
        let signal = Signal::Buy {
            price: 100.0,
            confidence: 0.8,
        };
        assert!(signal.is_buy());
        assert!(!signal.is_sell());
        assert!(!signal.is_hold());
        assert_eq!(signal.price(), Some(100.0));
        assert_eq!(signal.confidence(), Some(0.8));
    }

    #[test]
    fn test_signal_sell() {
        let signal = Signal::Sell {
            price: 100.0,
            confidence: 0.7,
        };
        assert!(!signal.is_buy());
        assert!(signal.is_sell());
        assert_eq!(signal.price(), Some(100.0));
    }

    #[test]
    fn test_signal_hold() {
        let signal = Signal::Hold;
        assert!(signal.is_hold());
        assert_eq!(signal.price(), None);
        assert_eq!(signal.confidence(), None);
    }
}