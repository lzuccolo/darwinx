//! Nodos del Abstract Syntax Tree para estrategias

use darwinx_core::TimeFrame;
use serde::{Deserialize, Serialize};

/// Estrategia completa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyAST {
    pub name: String,
    pub timeframe: TimeFrame,
    pub entry_rules: RuleSet,
    pub exit_rules: RuleSet,
}

/// Conjunto de reglas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSet {
    pub operator: LogicalOperator,
    pub conditions: Vec<Condition>,
}

/// Operador lógico
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogicalOperator {
    And,
    Or,
}

/// Condición individual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub indicator: IndicatorType,
    pub comparison: Comparison,
    pub value: ConditionValue,
}

/// 🎯 NUEVO: Indicador dinámico (funciona con cualquier indicador del registry)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorType {
    /// Nombre del indicador (ej: "sma", "rsi", "macd")
    pub name: String,
    
    /// Parámetros del indicador en orden
    /// Ejemplo: 
    /// - SMA: [20.0] (period)
    /// - MACD: [12.0, 26.0, 9.0] (fast, slow, signal)
    /// - Bollinger: [20.0, 2.0] (period, std_dev)
    pub params: Vec<f64>,
}

impl IndicatorType {
    /// Crea un nuevo indicador
    pub fn new(name: impl Into<String>, params: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            params,
        }
    }
    
    /// Constructor conveniente para indicadores con 1 parámetro (period)
    pub fn with_period(name: impl Into<String>, period: usize) -> Self {
        Self {
            name: name.into(),
            params: vec![period as f64],
        }
    }
    
    /// Retorna el nombre del indicador
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Retorna los parámetros
    pub fn params(&self) -> &[f64] {
        &self.params
    }
    
    /// Retorna una representación legible
    pub fn display(&self) -> String {
        if self.params.is_empty() {
            self.name.clone()
        } else {
            format!("{}({})", self.name, 
                self.params.iter()
                    .map(|p| format!("{:.1}", p))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

/// Comparación
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Comparison {
    GreaterThan,
    LessThan,
    CrossesAbove,
    CrossesBelow,
    Equals,
}

/// Valor de condición
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionValue {
    Number(f64),
    Indicator(IndicatorType),
    Price,
}

impl StrategyAST {
    pub fn new(name: String, timeframe: TimeFrame) -> Self {
        Self {
            name,
            timeframe,
            entry_rules: RuleSet {
                operator: LogicalOperator::And,
                conditions: Vec::new(),
            },
            exit_rules: RuleSet {
                operator: LogicalOperator::And,
                conditions: Vec::new(),
            },
        }
    }

    /// Retorna la complejidad de la estrategia
    pub fn complexity(&self) -> usize {
        self.entry_rules.conditions.len() + self.exit_rules.conditions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_creation() {
        let strategy = StrategyAST::new("Test".to_string(), TimeFrame::H1);
        assert_eq!(strategy.name, "Test");
        assert_eq!(strategy.complexity(), 0);
    }

    #[test]
    fn test_complexity() {
        let mut strategy = StrategyAST::new("Test".to_string(), TimeFrame::H1);
        
        strategy.entry_rules.conditions.push(Condition {
            indicator: IndicatorType::with_period("rsi", 14),
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Number(50.0),
        });
        
        assert_eq!(strategy.complexity(), 1);
    }

    #[test]
    fn test_indicator_display() {
        let sma = IndicatorType::with_period("sma", 20);
        assert_eq!(sma.display(), "sma(20.0)");
        
        let macd = IndicatorType::new("macd", vec![12.0, 26.0, 9.0]);
        assert_eq!(macd.display(), "macd(12.0, 26.0, 9.0)");
        
        let obv = IndicatorType::new("obv", vec![]);
        assert_eq!(obv.display(), "obv");
    }
}