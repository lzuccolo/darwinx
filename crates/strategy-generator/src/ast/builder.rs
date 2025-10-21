//! Constructor de estrategias con API fluida

use crate::ast::nodes::*;
use darwinx_core::TimeFrame;

/// Builder para construir estrategias programáticamente
pub struct StrategyBuilder {
    strategy: StrategyAST,
}

impl StrategyBuilder {
    pub fn new(name: String, timeframe: TimeFrame) -> Self {
        Self {
            strategy: StrategyAST::new(name, timeframe),
        }
    }

    /// Agrega una condición de entrada
    pub fn add_entry_condition(mut self, condition: Condition) -> Self {
        self.strategy.entry_rules.conditions.push(condition);
        self
    }

    /// Agrega una condición de salida
    pub fn add_exit_condition(mut self, condition: Condition) -> Self {
        self.strategy.exit_rules.conditions.push(condition);
        self
    }

    /// Define el operador lógico para las reglas de entrada
    pub fn entry_operator(mut self, operator: LogicalOperator) -> Self {
        self.strategy.entry_rules.operator = operator;
        self
    }

    /// Define el operador lógico para las reglas de salida
    pub fn exit_operator(mut self, operator: LogicalOperator) -> Self {
        self.strategy.exit_rules.operator = operator;
        self
    }

    /// Construye la estrategia final
    pub fn build(self) -> StrategyAST {
        self.strategy
    }
}

/// Helper para crear condiciones fácilmente
pub struct ConditionBuilder;

impl ConditionBuilder {
    /// RSI > valor
    pub fn rsi_above(period: usize, value: f64) -> Condition {
        Condition {
            indicator: IndicatorType::Rsi { period },
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Number(value),
        }
    }

    /// RSI < valor
    pub fn rsi_below(period: usize, value: f64) -> Condition {
        Condition {
            indicator: IndicatorType::Rsi { period },
            comparison: Comparison::LessThan,
            value: ConditionValue::Number(value),
        }
    }

    /// SMA cruza por encima de otra SMA
    pub fn sma_cross_above(fast: usize, slow: usize) -> Condition {
        Condition {
            indicator: IndicatorType::Sma { period: fast },
            comparison: Comparison::CrossesAbove,
            value: ConditionValue::Indicator(IndicatorType::Sma { period: slow }),
        }
    }

    /// SMA cruza por debajo de otra SMA
    pub fn sma_cross_below(fast: usize, slow: usize) -> Condition {
        Condition {
            indicator: IndicatorType::Sma { period: fast },
            comparison: Comparison::CrossesBelow,
            value: ConditionValue::Indicator(IndicatorType::Sma { period: slow }),
        }
    }

    /// Precio > SMA
    pub fn price_above_sma(period: usize) -> Condition {
        Condition {
            indicator: IndicatorType::Sma { period },
            comparison: Comparison::LessThan,
            value: ConditionValue::Price,
        }
    }

    /// ATR > valor
    pub fn atr_above(period: usize, value: f64) -> Condition {
        Condition {
            indicator: IndicatorType::Atr { period },
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Number(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_builder() {
        let strategy = StrategyBuilder::new("Golden Cross".to_string(), TimeFrame::H1)
            .add_entry_condition(ConditionBuilder::sma_cross_above(50, 200))
            .add_entry_condition(ConditionBuilder::rsi_above(14, 50.0))
            .entry_operator(LogicalOperator::And)
            .add_exit_condition(ConditionBuilder::sma_cross_below(50, 200))
            .exit_operator(LogicalOperator::Or)
            .build();

        assert_eq!(strategy.name, "Golden Cross");
        assert_eq!(strategy.entry_rules.conditions.len(), 2);
        assert_eq!(strategy.exit_rules.conditions.len(), 1);
        assert_eq!(strategy.entry_rules.operator, LogicalOperator::And);
    }

    #[test]
    fn test_condition_helpers() {
        let cond1 = ConditionBuilder::rsi_above(14, 70.0);
        assert!(matches!(cond1.indicator, IndicatorType::Rsi { period: 14 }));
        assert_eq!(cond1.comparison, Comparison::GreaterThan);

        let cond2 = ConditionBuilder::sma_cross_above(10, 30);
        assert_eq!(cond2.comparison, Comparison::CrossesAbove);
    }
}