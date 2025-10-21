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

/// Tipos de indicadores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndicatorType {
    Sma { period: usize },
    Ema { period: usize },
    Rsi { period: usize },
    Macd { fast: usize, slow: usize, signal: usize },
    BollingerBands { period: usize, std_dev: f64 },
    Atr { period: usize },
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
            indicator: IndicatorType::Rsi { period: 14 },
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Number(50.0),
        });
        
        assert_eq!(strategy.complexity(), 1);
    }
}