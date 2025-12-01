//! Constructor de estrategias con API fluida - Multi-Timeframe Edition
//!
//! ## Nuevas características v2.1:
//! - ✨ Multi-timeframe builder methods
//! - ✨ TimeframeCategory-aware condition builders
//! - ✨ Fluent API para estrategias multi-TF
//! - ✨ Enhanced ConditionBuilder con timeframe support

use crate::ast::nodes::*;
use darwinx_core::TimeFrame;

/// ✨ UPDATED: Builder para construir estrategias multi-timeframe programáticamente
pub struct StrategyBuilder {
    strategy: StrategyAST,
}

impl StrategyBuilder {
    /// Crea un nuevo builder con timeframe principal
    pub fn new(name: String, primary_timeframe: TimeFrame) -> Self {
        Self {
            strategy: StrategyAST::new(name, primary_timeframe),
        }
    }

    /// Agrega una condición de entrada (legacy method - usa Current timeframe)
    pub fn add_entry_condition(mut self, condition: Condition) -> Self {
        self.strategy.entry_rules.conditions.push(condition);
        self
    }

    /// ✨ NEW: Agrega una condición de entrada con timeframe específico
    pub fn add_entry_condition_with_timeframe(
        mut self, 
        mut condition: Condition, 
        timeframe_category: TimeframeCategory
    ) -> Self {
        condition.indicator.timeframe_category = timeframe_category;
        self.strategy.entry_rules.conditions.push(condition);
        self
    }

    /// Agrega una condición de salida (legacy method - usa Current timeframe)
    pub fn add_exit_condition(mut self, condition: Condition) -> Self {
        self.strategy.exit_rules.conditions.push(condition);
        self
    }

    /// ✨ NEW: Agrega una condición de salida con timeframe específico
    pub fn add_exit_condition_with_timeframe(
        mut self, 
        mut condition: Condition, 
        timeframe_category: TimeframeCategory
    ) -> Self {
        condition.indicator.timeframe_category = timeframe_category;
        self.strategy.exit_rules.conditions.push(condition);
        self
    }

    /// ✨ NEW: Agrega múltiples condiciones de entrada con diferentes timeframes
    pub fn add_entry_conditions_multi_tf(mut self, conditions: Vec<(Condition, TimeframeCategory)>) -> Self {
        for (mut condition, timeframe_category) in conditions {
            condition.indicator.timeframe_category = timeframe_category;
            self.strategy.entry_rules.conditions.push(condition);
        }
        self
    }

    /// ✨ NEW: Agrega múltiples condiciones de salida con diferentes timeframes
    pub fn add_exit_conditions_multi_tf(mut self, conditions: Vec<(Condition, TimeframeCategory)>) -> Self {
        for (mut condition, timeframe_category) in conditions {
            condition.indicator.timeframe_category = timeframe_category;
            self.strategy.exit_rules.conditions.push(condition);
        }
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

    /// ✨ NEW: Helper methods para crear estrategias multi-timeframe comunes

    /// Crea una estrategia Golden Cross multi-timeframe
    /// - EMA corto en Current timeframe
    /// - EMA largo en Medium timeframe
    /// - RSI filter en Current timeframe
    pub fn golden_cross_multi_tf(
        name: String,
        primary_timeframe: TimeFrame,
        short_period: usize,
        long_period: usize,
        rsi_period: usize,
    ) -> Self {
        Self::new(name, primary_timeframe)
            .add_entry_condition_with_timeframe(
                ConditionBuilder::crosses_above(
                    "ema", vec![short_period as f64],
                    "ema", vec![long_period as f64]
                ),
                TimeframeCategory::Current
            )
            .add_entry_condition_with_timeframe(
                ConditionBuilder::above("rsi", vec![rsi_period as f64], 50.0),
                TimeframeCategory::Current
            )
            .add_entry_condition_with_timeframe(
                ConditionBuilder::above("ema", vec![long_period as f64], 0.0), // Trend confirmation
                TimeframeCategory::Medium
            )
            .add_exit_condition_with_timeframe(
                ConditionBuilder::crosses_below(
                    "ema", vec![short_period as f64],
                    "ema", vec![long_period as f64]
                ),
                TimeframeCategory::Current
            )
            .add_exit_condition_with_timeframe(
                ConditionBuilder::above("rsi", vec![rsi_period as f64], 70.0),
                TimeframeCategory::Current
            )
            .entry_operator(LogicalOperator::And)
            .exit_operator(LogicalOperator::Or)
    }

    /// Crea una estrategia de Mean Reversion multi-timeframe
    /// - RSI oversold en Current timeframe
    /// - Price above SMA en Medium timeframe (trend filter)
    /// - Volume confirmation en Current timeframe
    pub fn mean_reversion_multi_tf(
        name: String,
        primary_timeframe: TimeFrame,
        rsi_period: usize,
        sma_period: usize,
        volume_period: usize,
    ) -> Self {
        Self::new(name, primary_timeframe)
            .add_entry_condition_with_timeframe(
                ConditionBuilder::below("rsi", vec![rsi_period as f64], 30.0),
                TimeframeCategory::Current
            )
            .add_entry_condition_with_timeframe(
                ConditionBuilder::above_price("sma", vec![sma_period as f64]),
                TimeframeCategory::Medium
            )
            .add_entry_condition_with_timeframe(
                ConditionBuilder::above(
                    "volume", vec![],
                    1.5 // 1.5x average volume
                ),
                TimeframeCategory::Current
            )
            .add_exit_condition_with_timeframe(
                ConditionBuilder::above("rsi", vec![rsi_period as f64], 50.0),
                TimeframeCategory::Current
            )
            .add_exit_condition_with_timeframe(
                ConditionBuilder::below_price("sma", vec![sma_period as f64]),
                TimeframeCategory::Medium
            )
            .entry_operator(LogicalOperator::And)
            .exit_operator(LogicalOperator::Or)
    }

    /// Construye la estrategia final
    pub fn build(self) -> StrategyAST {
        self.strategy
    }
}

/// ✨ UPDATED: Helper con soporte multi-timeframe para crear condiciones
/// 
/// Mantiene 100% backward compatibility mientras agrega nuevas capacidades multi-TF
pub struct ConditionBuilder;

impl ConditionBuilder {
    /// ✨ NEW: Crea una condición con timeframe específico
    pub fn above_with_timeframe(
        name: impl Into<String>, 
        params: Vec<f64>, 
        value: f64,
        timeframe_category: TimeframeCategory
    ) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params, timeframe_category),
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Number(value),
        }
    }

    /// Crea una condición: indicador > valor (usa Current timeframe por defecto)
    /// 
    /// # Ejemplos
    /// ```
    /// // RSI > 70
    /// ConditionBuilder::above("rsi", vec![14.0], 70.0);
    /// 
    /// // SMA > 100
    /// ConditionBuilder::above("sma", vec![20.0], 100.0);
    /// 
    /// // OBV > 1000000
    /// ConditionBuilder::above("obv", vec![], 1000000.0);
    /// ```
    pub fn above(name: impl Into<String>, params: Vec<f64>, value: f64) -> Condition {
        Self::above_with_timeframe(name, params, value, TimeframeCategory::Current)
    }
    
    /// ✨ NEW: Crea una condición: indicador < valor con timeframe específico
    pub fn below_with_timeframe(
        name: impl Into<String>, 
        params: Vec<f64>, 
        value: f64,
        timeframe_category: TimeframeCategory
    ) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params, timeframe_category),
            comparison: Comparison::LessThan,
            value: ConditionValue::Number(value),
        }
    }

    /// Crea una condición: indicador < valor (usa Current timeframe por defecto)
    pub fn below(name: impl Into<String>, params: Vec<f64>, value: f64) -> Condition {
        Self::below_with_timeframe(name, params, value, TimeframeCategory::Current)
    }
    
    /// Crea una condición: indicador = valor (usa Current timeframe por defecto)
    pub fn equals(name: impl Into<String>, params: Vec<f64>, value: f64) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params, TimeframeCategory::Current),
            comparison: Comparison::Equals,
            value: ConditionValue::Number(value),
        }
    }

    /// ✨ NEW: Crea una condición entre indicadores con timeframes específicos
    pub fn indicator_above_multi_tf(
        name1: impl Into<String>,
        params1: Vec<f64>,
        timeframe1: TimeframeCategory,
        name2: impl Into<String>,
        params2: Vec<f64>,
        timeframe2: TimeframeCategory,
    ) -> Condition {
        Condition {
            indicator: IndicatorType::new(name1, params1, timeframe1),
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Indicator(IndicatorType::new(name2, params2, timeframe2)),
        }
    }
    
    /// Crea una condición: indicador1 > indicador2 (ambos en Current timeframe)
    /// 
    /// # Ejemplos
    /// ```
    /// // SMA(10) > SMA(30)
    /// ConditionBuilder::indicator_above(
    ///     "sma", vec![10.0],
    ///     "sma", vec![30.0]
    /// );
    /// 
    /// // EMA(12) > EMA(26)
    /// ConditionBuilder::indicator_above(
    ///     "ema", vec![12.0],
    ///     "ema", vec![26.0]
    /// );
    /// ```
    pub fn indicator_above(
        name1: impl Into<String>,
        params1: Vec<f64>,
        name2: impl Into<String>,
        params2: Vec<f64>,
    ) -> Condition {
        Self::indicator_above_multi_tf(
            name1, params1, TimeframeCategory::Current,
            name2, params2, TimeframeCategory::Current
        )
    }
    
    /// ✨ NEW: Crea una condición: indicador1 < indicador2 con timeframes específicos
    pub fn indicator_below_multi_tf(
        name1: impl Into<String>,
        params1: Vec<f64>,
        timeframe1: TimeframeCategory,
        name2: impl Into<String>,
        params2: Vec<f64>,
        timeframe2: TimeframeCategory,
    ) -> Condition {
        Condition {
            indicator: IndicatorType::new(name1, params1, timeframe1),
            comparison: Comparison::LessThan,
            value: ConditionValue::Indicator(IndicatorType::new(name2, params2, timeframe2)),
        }
    }

    /// Crea una condición: indicador1 < indicador2 (ambos en Current timeframe)
    pub fn indicator_below(
        name1: impl Into<String>,
        params1: Vec<f64>,
        name2: impl Into<String>,
        params2: Vec<f64>,
    ) -> Condition {
        Self::indicator_below_multi_tf(
            name1, params1, TimeframeCategory::Current,
            name2, params2, TimeframeCategory::Current
        )
    }
    
    /// Crea una condición: indicador cruza por encima de valor (Current timeframe)
    pub fn crosses_above_value(name: impl Into<String>, params: Vec<f64>, value: f64) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params, TimeframeCategory::Current),
            comparison: Comparison::CrossesAbove,
            value: ConditionValue::Number(value),
        }
    }
    
    /// Crea una condición: indicador cruza por debajo de valor (Current timeframe)
    pub fn crosses_below_value(name: impl Into<String>, params: Vec<f64>, value: f64) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params, TimeframeCategory::Current),
            comparison: Comparison::CrossesBelow,
            value: ConditionValue::Number(value),
        }
    }

    /// ✨ NEW: Crea una condición de cruce con timeframes específicos
    pub fn crosses_above_multi_tf(
        name1: impl Into<String>,
        params1: Vec<f64>,
        timeframe1: TimeframeCategory,
        name2: impl Into<String>,
        params2: Vec<f64>,
        timeframe2: TimeframeCategory,
    ) -> Condition {
        Condition {
            indicator: IndicatorType::new(name1, params1, timeframe1),
            comparison: Comparison::CrossesAbove,
            value: ConditionValue::Indicator(IndicatorType::new(name2, params2, timeframe2)),
        }
    }

    /// Crea una condición: indicador1 cruza por encima de indicador2 (ambos Current)
    pub fn crosses_above(
        name1: impl Into<String>,
        params1: Vec<f64>,
        name2: impl Into<String>,
        params2: Vec<f64>,
    ) -> Condition {
        Self::crosses_above_multi_tf(
            name1, params1, TimeframeCategory::Current,
            name2, params2, TimeframeCategory::Current
        )
    }
    
    /// ✨ NEW: Crea una condición de cruce hacia abajo con timeframes específicos
    pub fn crosses_below_multi_tf(
        name1: impl Into<String>,
        params1: Vec<f64>,
        timeframe1: TimeframeCategory,
        name2: impl Into<String>,
        params2: Vec<f64>,
        timeframe2: TimeframeCategory,
    ) -> Condition {
        Condition {
            indicator: IndicatorType::new(name1, params1, timeframe1),
            comparison: Comparison::CrossesBelow,
            value: ConditionValue::Indicator(IndicatorType::new(name2, params2, timeframe2)),
        }
    }

    /// Crea una condición: indicador1 cruza por debajo de indicador2 (ambos Current)
    pub fn crosses_below(
        name1: impl Into<String>,
        params1: Vec<f64>,
        name2: impl Into<String>,
        params2: Vec<f64>,
    ) -> Condition {
        Self::crosses_below_multi_tf(
            name1, params1, TimeframeCategory::Current,
            name2, params2, TimeframeCategory::Current
        )
    }
    
    /// Crea una condición: indicador > precio actual (Current timeframe)
    pub fn above_price(name: impl Into<String>, params: Vec<f64>) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params, TimeframeCategory::Current),
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Price,
        }
    }
    
    /// ✨ NEW: Crea una condición: indicador > precio con timeframe específico
    pub fn above_price_with_timeframe(
        name: impl Into<String>, 
        params: Vec<f64>,
        timeframe_category: TimeframeCategory
    ) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params, timeframe_category),
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Price,
        }
    }

    /// Crea una condición: indicador < precio actual (Current timeframe)
    pub fn below_price(name: impl Into<String>, params: Vec<f64>) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params, TimeframeCategory::Current),
            comparison: Comparison::LessThan,
            value: ConditionValue::Price,
        }
    }

    /// ✨ NEW: Crea una condición: indicador < precio con timeframe específico
    pub fn below_price_with_timeframe(
        name: impl Into<String>, 
        params: Vec<f64>,
        timeframe_category: TimeframeCategory
    ) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params, timeframe_category),
            comparison: Comparison::LessThan,
            value: ConditionValue::Price,
        }
    }
    
    /// Crea una condición: precio cruza por encima del indicador (Current timeframe)
    pub fn price_crosses_above(name: impl Into<String>, params: Vec<f64>) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params, TimeframeCategory::Current),
            comparison: Comparison::CrossesBelow,  // Invertido: precio sube = indicador baja relativamente
            value: ConditionValue::Price,
        }
    }
    
    /// Crea una condición: precio cruza por debajo del indicador (Current timeframe)
    pub fn price_crosses_below(name: impl Into<String>, params: Vec<f64>) -> Condition {
        Condition {
            indicator: IndicatorType::new(name, params, TimeframeCategory::Current),
            comparison: Comparison::CrossesAbove,  // Invertido
            value: ConditionValue::Price,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_timeframe_builder() {
        let strategy = StrategyBuilder::new("Multi-TF Test".to_string(), TimeFrame::M5)
            .add_entry_condition_with_timeframe(
                ConditionBuilder::above("rsi", vec![14.0], 50.0),
                TimeframeCategory::Current
            )
            .add_entry_condition_with_timeframe(
                ConditionBuilder::above("ema", vec![200.0], 100.0),
                TimeframeCategory::Medium
            )
            .build();

        assert_eq!(strategy.name, "Multi-TF Test");
        assert_eq!(strategy.primary_timeframe, TimeFrame::M5);
        assert_eq!(strategy.entry_rules.conditions.len(), 2);
        assert!(strategy.is_multi_timeframe());
        assert_eq!(strategy.timeframe_count(), 2);
    }

    #[test]
    fn test_golden_cross_multi_tf() {
        let strategy = StrategyBuilder::golden_cross_multi_tf(
            "Golden Cross Multi-TF".to_string(),
            TimeFrame::H1,
            50, 200, 14
        );

        let built_strategy = strategy.build();
        assert_eq!(built_strategy.name, "Golden Cross Multi-TF");
        assert_eq!(built_strategy.primary_timeframe, TimeFrame::H1);
        assert!(built_strategy.is_multi_timeframe());
        assert_eq!(built_strategy.entry_rules.conditions.len(), 3);
        assert_eq!(built_strategy.exit_rules.conditions.len(), 2);
    }

    #[test]
    fn test_mean_reversion_multi_tf() {
        let strategy = StrategyBuilder::mean_reversion_multi_tf(
            "Mean Reversion Multi-TF".to_string(),
            TimeFrame::M15,
            14, 50, 20
        );

        let built_strategy = strategy.build();
        assert_eq!(built_strategy.name, "Mean Reversion Multi-TF");
        assert_eq!(built_strategy.primary_timeframe, TimeFrame::M15);
        assert!(built_strategy.is_multi_timeframe());
    }

    #[test]
    fn test_condition_builder_multi_tf() {
        let condition = ConditionBuilder::above_with_timeframe(
            "rsi", vec![14.0], 70.0, TimeframeCategory::Medium
        );
        
        assert_eq!(condition.indicator.name(), "rsi");
        assert_eq!(condition.indicator.timeframe_category(), TimeframeCategory::Medium);
        assert_eq!(condition.comparison, Comparison::GreaterThan);
    }

    #[test]
    fn test_multi_tf_indicator_conditions() {
        let condition = ConditionBuilder::indicator_above_multi_tf(
            "ema", vec![50.0], TimeframeCategory::Current,
            "sma", vec![200.0], TimeframeCategory::High
        );
        
        assert_eq!(condition.indicator.timeframe_category(), TimeframeCategory::Current);
        
        if let ConditionValue::Indicator(ind) = &condition.value {
            assert_eq!(ind.timeframe_category(), TimeframeCategory::High);
        } else {
            panic!("Expected Indicator value");
        }
    }

    #[test]
    fn test_crosses_multi_tf() {
        let condition = ConditionBuilder::crosses_above_multi_tf(
            "ema", vec![12.0], TimeframeCategory::Current,
            "ema", vec![26.0], TimeframeCategory::Medium
        );
        
        assert_eq!(condition.comparison, Comparison::CrossesAbove);
        assert_eq!(condition.indicator.timeframe_category(), TimeframeCategory::Current);
    }

    #[test]
    fn test_multiple_conditions_multi_tf() {
        let conditions = vec![
            (ConditionBuilder::above("rsi", vec![14.0], 50.0), TimeframeCategory::Current),
            (ConditionBuilder::above("ema", vec![200.0], 100.0), TimeframeCategory::High),
        ];

        let strategy = StrategyBuilder::new("Multi Conditions".to_string(), TimeFrame::M5)
            .add_entry_conditions_multi_tf(conditions)
            .build();

        assert_eq!(strategy.entry_rules.conditions.len(), 2);
        assert!(strategy.is_multi_timeframe());
    }

    #[test]
    fn test_backward_compatibility() {
        // Test que los métodos legacy siguen funcionando
        let strategy = StrategyBuilder::new("Legacy Test".to_string(), TimeFrame::H1)
            .add_entry_condition(ConditionBuilder::above("rsi", vec![14.0], 70.0))
            .add_exit_condition(ConditionBuilder::below("rsi", vec![14.0], 30.0))
            .build();

        assert_eq!(strategy.name, "Legacy Test");
        assert_eq!(strategy.entry_rules.conditions.len(), 1);
        assert_eq!(strategy.exit_rules.conditions.len(), 1);
        
        // Todos los indicadores deberían estar en Current timeframe por defecto
        assert!(!strategy.is_multi_timeframe());
    }

    #[test]
    fn test_price_conditions_with_timeframe() {
        let condition1 = ConditionBuilder::above_price_with_timeframe(
            "sma", vec![20.0], TimeframeCategory::Medium
        );
        
        assert_eq!(condition1.indicator.timeframe_category(), TimeframeCategory::Medium);
        
        let condition2 = ConditionBuilder::below_price_with_timeframe(
            "ema", vec![50.0], TimeframeCategory::High
        );
        
        assert_eq!(condition2.indicator.timeframe_category(), TimeframeCategory::High);
    }
}
