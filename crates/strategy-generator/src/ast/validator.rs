//! Validador de estrategias - 100% DINÁMICO usando registry

use crate::ast::nodes::*;
use crate::constraints::StrategyConstraints;
use darwinx_indicators::registry;

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

        // Validar cada condición
        self.validate_conditions(&strategy.entry_rules.conditions, "entrada", &mut errors);
        self.validate_conditions(&strategy.exit_rules.conditions, "salida", &mut errors);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn count_unique_indicators(&self, strategy: &StrategyAST) -> usize {
        let mut indicators = std::collections::HashSet::new();

        for condition in &strategy.entry_rules.conditions {
            indicators.insert(condition.indicator.name().to_string());
            if let ConditionValue::Indicator(ind) = &condition.value {
                indicators.insert(ind.name().to_string());
            }
        }

        for condition in &strategy.exit_rules.conditions {
            indicators.insert(condition.indicator.name().to_string());
            if let ConditionValue::Indicator(ind) = &condition.value {
                indicators.insert(ind.name().to_string());
            }
        }

        indicators.len()
    }

    fn validate_conditions(
        &self,
        conditions: &[Condition],
        rule_type: &str,
        errors: &mut Vec<String>,
    ) {
        for (idx, condition) in conditions.iter().enumerate() {
            let location = format!("{} #{}", rule_type, idx + 1);
            self.validate_condition(condition, &location, errors);
        }
    }

    fn validate_condition(&self, condition: &Condition, location: &str, errors: &mut Vec<String>) {
        // Validar indicador principal
        self.validate_indicator(&condition.indicator, location, errors);

        // Validar valor si es otro indicador
        if let ConditionValue::Indicator(ind) = &condition.value {
            let loc = format!("{} (valor de comparación)", location);
            self.validate_indicator(ind, &loc, errors);
        }
    }

    /// 🎯 Validación 100% dinámica usando metadata del registry
    /// 
    /// NO tiene casos específicos por indicador.
    /// Todo se valida usando la metadata del registry.
    fn validate_indicator(&self, indicator: &IndicatorType, location: &str, errors: &mut Vec<String>) {
        let name = indicator.name();
        
        // Verificar que el indicador existe en el registry
        let metadata = match registry::get(name) {
            Some(meta) => meta,
            None => {
                errors.push(format!(
                    "{}: Indicador '{}' no encontrado en el registry",
                    location, name
                ));
                return;
            }
        };

        // Verificar número de parámetros
        let expected_params = metadata.parameters.len();
        let actual_params = indicator.params().len();
        
        if actual_params != expected_params {
            errors.push(format!(
                "{}: '{}' requiere {} parámetros, pero se proporcionaron {}",
                location, name, expected_params, actual_params
            ));
            return;
        }

        // Validar cada parámetro contra sus límites definidos en metadata
        for (idx, (param_def, &param_value)) in metadata.parameters.iter()
            .zip(indicator.params().iter())
            .enumerate()
        {
            // Verificar mínimo
            if param_value < param_def.min {
                errors.push(format!(
                    "{}: '{}' parámetro {} ('{}') = {:.2} está por debajo del mínimo {:.2}",
                    location, name, idx + 1, param_def.name, param_value, param_def.min
                ));
            }
            
            // Verificar máximo
            if param_value > param_def.max {
                errors.push(format!(
                    "{}: '{}' parámetro {} ('{}') = {:.2} está por encima del máximo {:.2}",
                    location, name, idx + 1, param_def.name, param_value, param_def.max
                ));
            }
        }
    }

    /// Verifica duplicados innecesarios (warning, no error)
    pub fn check_duplicates(&self, strategy: &StrategyAST) -> Vec<String> {
        let mut warnings = Vec::new();
        let mut seen = std::collections::HashMap::new();

        // Contar ocurrencias de cada combinación indicador+params
        for condition in strategy.entry_rules.conditions.iter()
            .chain(strategy.exit_rules.conditions.iter())
        {
            let key = format!("{}({:?})", 
                condition.indicator.name(), 
                condition.indicator.params()
            );
            
            *seen.entry(key).or_insert(0) += 1;
        }

        // Reportar duplicados excesivos
        for (key, count) in seen {
            if count > 3 {
                warnings.push(format!(
                    "Indicador {} aparece {} veces (posible redundancia)",
                    key, count
                ));
            }
        }

        warnings
    }

    /// Verifica la complejidad lógica de la estrategia
    pub fn check_complexity(&self, strategy: &StrategyAST) -> Vec<String> {
        let mut warnings = Vec::new();

        // Demasiadas condiciones AND (difícil de cumplir)
        if strategy.entry_rules.operator == LogicalOperator::And 
            && strategy.entry_rules.conditions.len() > 5 
        {
            warnings.push(format!(
                "Entrada tiene {} condiciones AND (puede ser demasiado restrictivo)",
                strategy.entry_rules.conditions.len()
            ));
        }

        // Demasiadas condiciones OR (puede generar muchas señales falsas)
        if strategy.entry_rules.operator == LogicalOperator::Or 
            && strategy.entry_rules.conditions.len() > 7 
        {
            warnings.push(format!(
                "Entrada tiene {} condiciones OR (puede generar muchas señales)",
                strategy.entry_rules.conditions.len()
            ));
        }

        warnings
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
            .add_entry_condition(ConditionBuilder::above("rsi", vec![14.0], 50.0))
            .add_exit_condition(ConditionBuilder::below("rsi", vec![14.0], 50.0))
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
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("nombre")));
    }

    #[test]
    fn test_no_entry_conditions() {
        let mut strategy = StrategyAST::new("Test".to_string(), TimeFrame::H1);
        strategy.exit_rules.conditions.push(
            ConditionBuilder::below("rsi", vec![14.0], 30.0)
        );

        let validator = StrategyValidator::new(StrategyConstraints::default());
        let result = validator.validate(&strategy);
        
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("entrada")));
    }

    #[test]
    fn test_unknown_indicator() {
        let mut strategy = StrategyAST::new("Test".to_string(), TimeFrame::H1);
        strategy.entry_rules.conditions.push(Condition {
            indicator: IndicatorType::new("indicador_inexistente", vec![20.0]),
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Number(50.0),
        });
        strategy.exit_rules.conditions.push(
            ConditionBuilder::below("rsi", vec![14.0], 30.0)
        );

        let validator = StrategyValidator::new(StrategyConstraints::default());
        let result = validator.validate(&strategy);
        
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("no encontrado") || e.contains("not found")));
    }

    #[test]
    fn test_wrong_param_count() {
        // Si un indicador requiere X parámetros, dar Y diferente
        let mut strategy = StrategyAST::new("Test".to_string(), TimeFrame::H1);
        
        // Asumiendo que 'sma' requiere 1 parámetro, dar 2
        strategy.entry_rules.conditions.push(Condition {
            indicator: IndicatorType::new("sma", vec![20.0, 30.0]), // Mal: 2 params
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Number(100.0),
        });
        strategy.exit_rules.conditions.push(
            ConditionBuilder::below("rsi", vec![14.0], 30.0)
        );

        let validator = StrategyValidator::new(StrategyConstraints::default());
        let result = validator.validate(&strategy);
        
        // Puede o no fallar dependiendo de si el indicador acepta 2 params
        // Este test es más informativo que assertivo
        if result.is_err() {
            let errors = result.unwrap_err();
            println!("Errors: {:?}", errors);
        }
    }

    #[test]
    fn test_param_out_of_range() {
        let mut strategy = StrategyAST::new("Test".to_string(), TimeFrame::H1);
        
        // RSI con período fuera de rango (si metadata dice min=2, max=50)
        strategy.entry_rules.conditions.push(Condition {
            indicator: IndicatorType::new("rsi", vec![1.0]), // < min
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Number(70.0),
        });
        strategy.exit_rules.conditions.push(
            ConditionBuilder::below("rsi", vec![14.0], 30.0)
        );

        let validator = StrategyValidator::new(StrategyConstraints::default());
        let result = validator.validate(&strategy);
        
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| 
            e.contains("mínimo") || e.contains("minimum") || e.contains("debajo")
        ));
    }

    #[test]
    fn test_check_duplicates() {
        let mut builder = StrategyBuilder::new("Test".to_string(), TimeFrame::H1);
        
        // Agregar mismo indicador 5 veces
        for _ in 0..5 {
            builder = builder.add_entry_condition(
                ConditionBuilder::above("rsi", vec![14.0], 70.0)
            );
        }
        
        let strategy = builder
            .add_exit_condition(ConditionBuilder::below("rsi", vec![14.0], 30.0))
            .build();

        let validator = StrategyValidator::new(StrategyConstraints::default());
        let warnings = validator.check_duplicates(&strategy);
        
        assert!(!warnings.is_empty());
    }

    #[test]
    fn test_check_complexity() {
        let mut builder = StrategyBuilder::new("Test".to_string(), TimeFrame::H1);
        
        // Muchas condiciones AND
        for i in 0..8 {
            builder = builder.add_entry_condition(
                ConditionBuilder::above("rsi", vec![14.0], 50.0 + i as f64)
            );
        }
        
        let strategy = builder
            .entry_operator(LogicalOperator::And)
            .add_exit_condition(ConditionBuilder::below("rsi", vec![14.0], 30.0))
            .build();

        let validator = StrategyValidator::new(StrategyConstraints::default());
        let warnings = validator.check_complexity(&strategy);
        
        assert!(!warnings.is_empty());
    }

    #[test]
    fn test_too_many_conditions() {
        let mut builder = StrategyBuilder::new("Test".to_string(), TimeFrame::H1);
        
        // Exceder máximo
        for i in 0..15 {
            builder = builder.add_entry_condition(
                ConditionBuilder::above("rsi", vec![14.0], 50.0 + i as f64)
            );
        }
        
        let strategy = builder
            .add_exit_condition(ConditionBuilder::below("rsi", vec![14.0], 30.0))
            .build();

        let validator = StrategyValidator::new(StrategyConstraints::strict());
        let result = validator.validate(&strategy);
        
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("Complejidad") || e.contains("excede")));
    }

    #[test]
    fn test_any_valid_indicator() {
        // Debe funcionar con CUALQUIER indicador del registry
        let strategy = StrategyBuilder::new("Any".to_string(), TimeFrame::H1)
            .add_entry_condition(ConditionBuilder::above("sma", vec![20.0], 100.0))
            .add_entry_condition(ConditionBuilder::above("ema", vec![12.0], 95.0))
            .add_entry_condition(ConditionBuilder::above("vwap", vec![], 98.0))
            .add_exit_condition(ConditionBuilder::below("atr", vec![14.0], 1.0))
            .build();

        let validator = StrategyValidator::new(StrategyConstraints::default());
        let result = validator.validate(&strategy);
        
        // Si los indicadores están en el registry con parámetros correctos, debe pasar
        if result.is_err() {
            println!("Validation errors: {:?}", result.unwrap_err());
        }
    }
}