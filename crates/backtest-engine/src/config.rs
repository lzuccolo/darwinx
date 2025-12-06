//! Configuración del Backtest Engine

use serde::{Deserialize, Serialize};

/// Configuración para ejecutar un backtest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestConfig {
    /// Balance inicial
    pub initial_balance: f64,
    /// Comisión por trade (como porcentaje, ej: 0.001 = 0.1%)
    pub commission_rate: f64,
    /// Slippage en basis points (ej: 5 = 0.05%)
    pub slippage_bps: f64,
    /// Máximo número de posiciones simultáneas
    pub max_positions: usize,
    /// Riesgo por trade como porcentaje del balance (ej: 0.02 = 2%)
    pub risk_per_trade: f64,
    /// Stop loss como porcentaje del precio de entrada (ej: 0.02 = 2%, None = deshabilitado)
    pub stop_loss_percent: Option<f64>,
    /// Take profit como porcentaje del precio de entrada (ej: 0.05 = 5%, None = deshabilitado)
    pub take_profit_percent: Option<f64>,
    /// Porcentaje del balance a usar por posición (ej: 0.5 = 50%, 0.95 = 95%)
    pub position_size_percent: f64,
}

impl Default for BacktestConfig {
    fn default() -> Self {
        Self {
            initial_balance: 10000.0,
            commission_rate: 0.001, // 0.1%
            slippage_bps: 5.0,      // 0.05%
            max_positions: 1,
            risk_per_trade: 0.02,   // 2%
            stop_loss_percent: None, // Deshabilitado por defecto
            take_profit_percent: None, // Deshabilitado por defecto
            position_size_percent: 0.5, // 50% del balance por defecto
        }
    }
}

impl BacktestConfig {
    /// Crea una configuración con valores personalizados
    pub fn new(
        initial_balance: f64,
        commission_rate: f64,
        slippage_bps: f64,
        max_positions: usize,
        risk_per_trade: f64,
    ) -> Self {
        Self {
            initial_balance,
            commission_rate,
            slippage_bps,
            max_positions,
            risk_per_trade,
            stop_loss_percent: None,
            take_profit_percent: None,
            position_size_percent: 0.5,
        }
    }

    /// Crea una configuración con stop loss y take profit
    pub fn with_risk_management(
        initial_balance: f64,
        commission_rate: f64,
        slippage_bps: f64,
        max_positions: usize,
        risk_per_trade: f64,
        stop_loss_percent: Option<f64>,
        take_profit_percent: Option<f64>,
    ) -> Self {
        Self {
            initial_balance,
            commission_rate,
            slippage_bps,
            max_positions,
            risk_per_trade,
            stop_loss_percent,
            take_profit_percent,
            position_size_percent: 0.5,
        }
    }

    /// Crea una configuración completa con todos los parámetros
    pub fn with_position_size(
        initial_balance: f64,
        commission_rate: f64,
        slippage_bps: f64,
        max_positions: usize,
        risk_per_trade: f64,
        stop_loss_percent: Option<f64>,
        take_profit_percent: Option<f64>,
        position_size_percent: f64,
    ) -> Self {
        Self {
            initial_balance,
            commission_rate,
            slippage_bps,
            max_positions,
            risk_per_trade,
            stop_loss_percent,
            take_profit_percent,
            position_size_percent,
        }
    }

    /// Calcula la comisión para un trade
    pub fn calculate_commission(&self, trade_value: f64) -> f64 {
        trade_value * self.commission_rate
    }

    /// Calcula el slippage para un trade
    pub fn calculate_slippage(&self, price: f64) -> f64 {
        price * (self.slippage_bps / 10000.0)
    }
}

