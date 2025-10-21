//! Validador de estrategias

use crate::ast::nodes::*;
use crate::constraints::StrategyConstraints;

/// Resultado de validación
pub type ValidationResult = Result<(), Vec<String>>;

/// Validador de estrategias
pub struct StrategyValidator {
    constraints: StrategyConstraints,
}

impl StrategyValidator {
    pub fn new(constraints: StrategyConstraints) -> Self {
        Self { constraints }
    }

    /// Valida una estrategia completa
    pub fn validate(&self, strategy: &StrategyAST) -> ValidationResult {
        let mut errors = Vec::new();

        // Validar nombre
        if strategy.name.is_empty() {
            errors.push("El nombre no puede estar vacío".to_string());
        }

        // Validar complejidad
        if strategy.complexity() > self.constraints.max_conditions {
            errors.push(format!(
                "Complejidad {} excede el máximo permitido {}",
                strategy.complexity(),
                self.constraints.max_conditions
            ));
        }

        // Validar reglas de entrada
        if strategy.entry_rules.conditions.is_empty() {
            errors.push("Debe tener al menos una condición de entrada".to_string());
        }

        // Validar reglas de salida
        if strategy.exit_rules.conditions.is_empty() {
            errors.push("Debe tener al menos una condición de salida".to_string());
        }

        // Validar indicadores
        let indicator_count = self.count_unique_indicators(strategy);
        if indicator_count > self.constraints.max_indicators {
            errors.push(format!(
                "Número de indicadores {} excede el máximo {}",
                indicator_count, self.constraints.max_indicators
            ));
        }

        // Validar períodos de indicadores
        self.validate_indicator_periods(strategy, &mut errors);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn count_unique_indicators(&self, strategy: &StrategyAST) -> usize {
        let mut indicators = std::collections::HashSet::new();

        for condition in &strategy.entry_rules.conditions {
            indicators.insert(indicator_type_name(&condition.indicator));
            if let ConditionValue::Indicator(ind) = &condition.value {
                indicators.insert(indicator_type_name(ind));
            }
        }

        for condition in &strategy.exit_rules.conditions {
            indicators.insert(indicator_type_name(&condition.indicator));
            if let ConditionValue::Indicator(ind) = &condition.value {
                indicators.insert(indicator_type_name(ind));
            }
        }

        indicators.len()
    }

    fn validate_indicator_periods(&self, strategy: &StrategyAST, errors: &mut Vec<String>) {
        let all_conditions: Vec<_> = strategy
            .entry_rules
            .conditions
            .iter()
            .chain(strategy.exit_rules.conditions.iter())
            .collect();

        for condition in all_conditions {
            self.validate_indicator(&condition.indicator, errors);
            if let ConditionValue::Indicator(ind) = &condition.value {
                self.validate_indicator(ind, errors);
            }
        }
    }

    fn validate_indicator(&self, indicator: &IndicatorType, errors: &mut Vec<String>) {
        match indicator {
            IndicatorType::Sma { period } | IndicatorType::Ema { period } => {
                if *period < 2 {
                    errors.push(format!("Período de MA debe ser >= 2, es {}", period));
                }
                if *period > 200 {
                    errors.push(format!("Período de MA debe ser <= 200, es {}", period));
                }
            }
            IndicatorType::Rsi { period } => {
                if *period < 2 {
                    errors.push(format!("Período de RSI debe ser >= 2, es {}", period));
                }
                if *period > 50 {
                    errors.push(format!("Período de RSI debe ser <= 50, es {}", period));
                }
            }
            IndicatorType::Macd { fast, slow, signal } => {
                if fast >= slow {
                    errors.push(format!("MACD: fast ({}) debe ser < slow ({})", fast, slow));
                }
                if *signal < 2 {
                    errors.push(format!("MACD: signal debe ser >= 2, es {}", signal));
                }
            }
            IndicatorType::BollingerBands { period, std_dev } => {
                if *period < 5 {
                    errors.push(format!("BB: período debe ser >= 5, es {}", period));
                }
                if *std_dev <= 0.0 || *std_dev > 5.0 {
                    errors.push(format!("BB: std_dev debe estar entre 0 y 5, es {}", std_dev));
                }
            }
            IndicatorType::Atr { period } => {
                if *period < 5 {
                    errors.push(format!("ATR: período debe ser >= 5, es {}", period));
                }
            }
        }
    }
}

fn indicator_type_name(indicator: &IndicatorType) -> &'static str {
    match indicator {
        IndicatorType::Sma { .. } => "SMA",
        IndicatorType::Ema { .. } => "EMA",
        IndicatorType::Rsi { .. } => "RSI",
        IndicatorType::Macd { .. } => "MACD",
        IndicatorType::BollingerBands { .. } => "BB",
        IndicatorType::Atr { .. } => "ATR",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::builder::*;
    use darwinx_core::TimeFrame;

    #[test]
    fn test_valid_strategy() {
        let strategy = StrategyBuilder::new("Test".to_string(), TimeFrame::H1)
            .add_entry_condition(ConditionBuilder::rsi_above(14, 50.0))
            .add_exit_condition(ConditionBuilder::rsi_below(14, 50.0))
            .build();

        let validator = StrategyValidator::new(StrategyConstraints::default());
        assert!(validator.validate(&strategy).is_ok());
    }

    #[test]
    fn test_empty_name() {
        let strategy = StrategyAST::new("".to_string(), TimeFrame::H1);
        let validator = StrategyValidator::new(StrategyConstraints::default());
        
        let result = validator.validate(&strategy);
        assert!(result.is_err());
    }

    #[test]
    fn test_no_entry_conditions() {
        let mut strategy = StrategyAST::new("Test".to_string(), TimeFrame::H1);
        strategy.exit_rules.conditions.push(ConditionBuilder::rsi_below(14, 30.0));

        let validator = StrategyValidator::new(StrategyConstraints::default());
        let result = validator.validate(&strategy);
        
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("entrada")));
    }

    #[test]
    fn test_invalid_period() {
        let strategy = StrategyBuilder::new("Test".to_string(), TimeFrame::H1)
            .add_entry_condition(Condition {
                indicator: IndicatorType::Rsi { period: 1 },
                comparison: Comparison::GreaterThan,
                value: ConditionValue::Number(50.0),
            })
            .add_exit_condition(ConditionBuilder::rsi_below(14, 30.0))
            .build();

        let validator = StrategyValidator::new(StrategyConstraints::default());
        let result = validator.validate(&strategy);
        
        assert!(result.is_err());
    }
}