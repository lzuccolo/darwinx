//! Constructor de estrategias con API fluida - 100% DINÁMICO

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

/// Helper 100% dinámico para crear condiciones
/// 
/// NO incluye funciones específicas por indicador.
/// Usa solo métodos genéricos que funcionan con cualquier indicador del registry.
pub struct ConditionBuilder;

impl ConditionBuilder {
    /// Crea una condición: indicador > valor
    /// 
    /// # Ejemplos
    /// ```
    /// // RSI > 70
    //  ConditionBuilder::above("rsi", vec![14.0], 70.0);
    /// 
    /// // SMA > 100
    // ConditionBuilder::above("sma", vec![20.0], 100.0);
    /// 
    /// // OBV > 1000000
    // ConditionBuilder::above("obv", vec![], 1000000.0);
    /// ```
    pub fn above(name: impl Into<String>, params: Vec<f64>, value: f64) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params),
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Number(value),
        }
    }
    
    /// Crea una condición: indicador < valor
    pub fn below(name: impl Into<String>, params: Vec<f64>, value: f64) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params),
            comparison: Comparison::LessThan,
            value: ConditionValue::Number(value),
        }
    }
    
    /// Crea una condición: indicador = valor
    pub fn equals(name: impl Into<String>, params: Vec<f64>, value: f64) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params),
            comparison: Comparison::Equals,
            value: ConditionValue::Number(value),
        }
    }
    
    /// Crea una condición: indicador1 > indicador2
    /// 
    /// # Ejemplos
    // ```
    /// // SMA(10) > SMA(30)
    // ConditionBuilder::indicator_above(
    //     "sma", vec![10.0],
    //     "sma", vec![30.0]
    // );
    /// 
    // EMA(12) > EMA(26)
    // ConditionBuilder::indicator_above(
    //     "ema", vec![12.0],
    //     "ema", vec![26.0]
    // );
    // ```
    pub fn indicator_above(
        name1: impl Into<String>,
        params1: Vec<f64>,
        name2: impl Into<String>,
        params2: Vec<f64>,
    ) -> Condition {
        Condition {
            indicator: IndicatorType::new(name1, params1),
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Indicator(IndicatorType::new(name2, params2)),
        }
    }
    
    /// Crea una condición: indicador1 < indicador2
    pub fn indicator_below(
        name1: impl Into<String>,
        params1: Vec<f64>,
        name2: impl Into<String>,
        params2: Vec<f64>,
    ) -> Condition {
        Condition {
            indicator: IndicatorType::new(name1, params1),
            comparison: Comparison::LessThan,
            value: ConditionValue::Indicator(IndicatorType::new(name2, params2)),
        }
    }
    
    /// Crea una condición: indicador cruza por encima de valor
    pub fn crosses_above_value(name: impl Into<String>, params: Vec<f64>, value: f64) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params),
            comparison: Comparison::CrossesAbove,
            value: ConditionValue::Number(value),
        }
    }
    
    /// Crea una condición: indicador cruza por debajo de valor
    pub fn crosses_below_value(name: impl Into<String>, params: Vec<f64>, value: f64) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params),
            comparison: Comparison::CrossesBelow,
            value: ConditionValue::Number(value),
        }
    }
    

    pub fn crosses_above(
        name1: impl Into<String>,
        params1: Vec<f64>,
        name2: impl Into<String>,
        params2: Vec<f64>,
    ) -> Condition {
        Condition {
            indicator: IndicatorType::new(name1, params1),
            comparison: Comparison::CrossesAbove,
            value: ConditionValue::Indicator(IndicatorType::new(name2, params2)),
        }
    }
    
    /// Crea una condición: indicador1 cruza por debajo de indicador2
    pub fn crosses_below(
        name1: impl Into<String>,
        params1: Vec<f64>,
        name2: impl Into<String>,
        params2: Vec<f64>,
    ) -> Condition {
        Condition {
            indicator: IndicatorType::new(name1, params1),
            comparison: Comparison::CrossesBelow,
            value: ConditionValue::Indicator(IndicatorType::new(name2, params2)),
        }
    }
    
    /// Crea una condición: indicador > precio actual
    pub fn above_price(name: impl Into<String>, params: Vec<f64>) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params),
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Price,
        }
    }
    
    /// Crea una condición: indicador < precio actual
    pub fn below_price(name: impl Into<String>, params: Vec<f64>) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params),
            comparison: Comparison::LessThan,
            value: ConditionValue::Price,
        }
    }
    
    /// Crea una condición: precio cruza por encima del indicador
    pub fn price_crosses_above(name: impl Into<String>, params: Vec<f64>) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params),
            comparison: Comparison::CrossesBelow,  // Invertido: precio sube = indicador baja relativamente
            value: ConditionValue::Price,
        }
    }
    
    /// Crea una condición: precio cruza por debajo del indicador
    pub fn price_crosses_below(name: impl Into<String>, params: Vec<f64>) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params),
            comparison: Comparison::CrossesAbove,  // Invertido
            value: ConditionValue::Price,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_builder_basic() {
        let strategy = StrategyBuilder::new("Test Strategy".to_string(), TimeFrame::H1)
            .add_entry_condition(ConditionBuilder::above("rsi", vec![14.0], 70.0))
            .add_exit_condition(ConditionBuilder::below("rsi", vec![14.0], 30.0))
            .build();

        assert_eq!(strategy.name, "Test Strategy");
        assert_eq!(strategy.timeframe, TimeFrame::H1);
        assert_eq!(strategy.entry_rules.conditions.len(), 1);
        assert_eq!(strategy.exit_rules.conditions.len(), 1);
    }

    #[test]
    fn test_golden_cross_strategy() {
        let strategy = StrategyBuilder::new("Golden Cross".to_string(), TimeFrame::H1)
            .add_entry_condition(
                ConditionBuilder::crosses_above(
                    "sma", vec![50.0],
                    "sma", vec![200.0]
                )
            )
            .add_exit_condition(
                ConditionBuilder::crosses_below(
                    "sma", vec![50.0],
                    "sma", vec![200.0]
                )
            )
            .build();

        assert_eq!(strategy.name, "Golden Cross");
        assert_eq!(strategy.entry_rules.conditions[0].indicator.name(), "sma");
        assert_eq!(strategy.entry_rules.conditions[0].comparison, Comparison::CrossesAbove);
    }

    #[test]
    fn test_multi_indicator_strategy() {
        let strategy = StrategyBuilder::new("Multi".to_string(), TimeFrame::H1)
            .add_entry_condition(ConditionBuilder::above("rsi", vec![14.0], 50.0))
            .add_entry_condition(ConditionBuilder::above("obv", vec![], 1000000.0))
            .add_entry_condition(ConditionBuilder::below("mfi", vec![14.0], 80.0))
            .entry_operator(LogicalOperator::And)
            .add_exit_condition(ConditionBuilder::below("rsi", vec![14.0], 30.0))
            .add_exit_condition(ConditionBuilder::below("obv", vec![], 500000.0))
            .exit_operator(LogicalOperator::Or)
            .build();

        assert_eq!(strategy.entry_rules.conditions.len(), 3);
        assert_eq!(strategy.exit_rules.conditions.len(), 2);
        assert_eq!(strategy.entry_rules.operator, LogicalOperator::And);
        assert_eq!(strategy.exit_rules.operator, LogicalOperator::Or);
    }

    #[test]
    fn test_price_based_conditions() {
        let strategy = StrategyBuilder::new("Price Test".to_string(), TimeFrame::H1)
            .add_entry_condition(ConditionBuilder::price_crosses_above("sma", vec![20.0]))
            .add_exit_condition(ConditionBuilder::price_crosses_below("sma", vec![20.0]))
            .build();

        assert_eq!(strategy.entry_rules.conditions.len(), 1);
        if let ConditionValue::Price = strategy.entry_rules.conditions[0].value {
            // OK
        } else {
            panic!("Expected Price value");
        }
    }

    #[test]
    fn test_any_indicator_from_registry() {
        // Debe funcionar con CUALQUIER indicador
        let strategy = StrategyBuilder::new("Any Indicator".to_string(), TimeFrame::H1)
            .add_entry_condition(ConditionBuilder::above("vwap", vec![], 100.0))
            .add_entry_condition(ConditionBuilder::above("atr", vec![14.0], 1.5))
            .add_entry_condition(
                ConditionBuilder::crosses_above(
                    "ema", vec![12.0],
                    "ema", vec![26.0]
                )
            )
            .build();

        assert_eq!(strategy.entry_rules.conditions.len(), 3);
        
        // Verificar que los nombres son correctos
        assert_eq!(strategy.entry_rules.conditions[0].indicator.name(), "vwap");
        assert_eq!(strategy.entry_rules.conditions[1].indicator.name(), "atr");
        assert_eq!(strategy.entry_rules.conditions[2].indicator.name(), "ema");
    }

    #[test]
    fn test_complex_strategy() {
        let strategy = StrategyBuilder::new("Complex".to_string(), TimeFrame::H4)
            .add_entry_condition(
                ConditionBuilder::indicator_above(
                    "ema", vec![12.0],
                    "sma", vec![26.0]
                )
            )
            .add_entry_condition(ConditionBuilder::above("rsi", vec![14.0], 50.0))
            .add_entry_condition(ConditionBuilder::below("mfi", vec![14.0], 80.0))
            .add_entry_condition(ConditionBuilder::above_price("vwap", vec![]))
            .entry_operator(LogicalOperator::And)
            .add_exit_condition(ConditionBuilder::below("rsi", vec![14.0], 30.0))
            .add_exit_condition(ConditionBuilder::above("rsi", vec![14.0], 70.0))
            .add_exit_condition(ConditionBuilder::crosses_below_value("macd", vec![12.0, 26.0, 9.0], 0.0))
            .exit_operator(LogicalOperator::Or)
            .build();

        assert_eq!(strategy.complexity(), 7);
    }
}