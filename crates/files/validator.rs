//! Validador de estrategias multi-timeframe - 100% DINÁMICO usando registry
//!
//! ## Nuevas características v2.1:
//! - ✨ Multi-timeframe consistency validation
//! - ✨ TimeframeCategory validation 
//! - ✨ Cross-timeframe indicator validation
//! - ✨ Enhanced constraint checking with semantic analysis

use crate::ast::nodes::*;
use crate::constraints::strategy::StrategyConstraints;
use darwinx_indicators::registry;
use std::collections::{HashMap, HashSet};

/// Resultado de validación
pub type ValidationResult = Result<(), Vec<String>>;

/// ✨ NEW: Reporte de validación extendido
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub info: Vec<String>,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            info: Vec::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn add_error(&mut self, message: String) {
        self.errors.push(message);
    }

    pub fn add_warning(&mut self, message: String) {
        self.warnings.push(message);
    }

    pub fn add_info(&mut self, message: String) {
        self.info.push(message);
    }

    /// Convierte el reporte a ValidationResult (solo errores)
    pub fn to_result(&self) -> ValidationResult {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }
}

/// ✨ UPDATED: Validador de estrategias multi-timeframe
pub struct StrategyValidator {
    constraints: StrategyConstraints,
}

impl StrategyValidator {
    pub fn new(constraints: StrategyConstraints) -> Self {
        Self { constraints }
    }

    /// ✨ NEW: Validación completa con reporte detallado
    pub fn validate_detailed(&self, strategy: &StrategyAST) -> ValidationReport {
        let mut report = ValidationReport::new();

        // Validaciones básicas
        self.validate_basic_strategy(strategy, &mut report);
        
        // ✨ NEW: Validaciones multi-timeframe
        self.validate_multi_timeframe(strategy, &mut report);
        
        // Validaciones de indicadores
        self.validate_all_indicators(strategy, &mut report);
        
        // Análisis de complejidad y duplicados
        self.analyze_strategy_quality(strategy, &mut report);

        report
    }

    /// Valida una estrategia completa (método legacy - mantiene compatibilidad)
    pub fn validate(&self, strategy: &StrategyAST) -> ValidationResult {
        self.validate_detailed(strategy).to_result()
    }

    /// Validaciones básicas de la estrategia
    fn validate_basic_strategy(&self, strategy: &StrategyAST, report: &mut ValidationReport) {
        // Validar nombre
        if strategy.name.is_empty() {
            report.add_error("El nombre no puede estar vacío".to_string());
        }

        // Validar complejidad
        if strategy.complexity() > self.constraints.max_conditions {
            report.add_error(format!(
                "Complejidad {} excede el máximo permitido {}",
                strategy.complexity(),
                self.constraints.max_conditions
            ));
        }

        // Validar reglas de entrada
        if strategy.entry_rules.conditions.is_empty() {
            report.add_error("Debe tener al menos una condición de entrada".to_string());
        }

        // Validar reglas de salida
        if strategy.exit_rules.conditions.is_empty() {
            report.add_error("Debe tener al menos una condición de salida".to_string());
        }

        // Validar número de indicadores
        let indicator_count = self.count_unique_indicators(strategy);
        if indicator_count > self.constraints.max_indicators {
            report.add_error(format!(
                "Número de indicadores {} excede el máximo {}",
                indicator_count, self.constraints.max_indicators
            ));
        }

        // Info básica
        report.add_info(format!(
            "Strategy: {} (Primary TF: {:?})", 
            strategy.name, strategy.primary_timeframe
        ));
        report.add_info(format!(
            "Complexity: {} conditions ({} entry, {} exit)",
            strategy.complexity(),
            strategy.entry_rules.conditions.len(),
            strategy.exit_rules.conditions.len()
        ));
    }

    /// ✨ NEW: Validaciones específicas multi-timeframe
    fn validate_multi_timeframe(&self, strategy: &StrategyAST, report: &mut ValidationReport) {
        let used_categories = strategy.used_timeframe_categories();
        let timeframe_count = used_categories.len();

        // Validar límite de timeframes
        if timeframe_count > self.constraints.max_timeframes {
            report.add_error(format!(
                "Número de timeframes {} excede el máximo {}",
                timeframe_count, self.constraints.max_timeframes
            ));
        }

        // Info sobre timeframes utilizados
        if strategy.is_multi_timeframe() {
            report.add_info("✨ Multi-timeframe strategy detected".to_string());
            
            let mapping = strategy.timeframe_mapping();
            for (category, timeframe) in mapping {
                report.add_info(format!(
                    "  {} timeframe: {:?}",
                    category.display_name(), timeframe
                ));
            }

            // Estadísticas por timeframe
            let stats = strategy.indicator_stats_by_timeframe();
            for (category, count) in stats {
                report.add_info(format!(
                    "  {} indicators in {} timeframe",
                    count, category.display_name()
                ));
            }
        } else {
            report.add_info("Single timeframe strategy".to_string());
        }

        // Validar consistencia de timeframes
        self.validate_timeframe_consistency(strategy, report);

        // Warnings para configuraciones sub-óptimas
        self.check_multi_timeframe_warnings(strategy, report);
    }

    /// ✨ NEW: Validar consistencia de timeframes
    fn validate_timeframe_consistency(&self, strategy: &StrategyAST, report: &mut ValidationReport) {
        let mapping = strategy.timeframe_mapping();
        
        // Verificar que no hay timeframes duplicados innecesarios
        let mut timeframe_frequency: HashMap<darwinx_core::TimeFrame, usize> = HashMap::new();
        for timeframe in mapping.values() {
            *timeframe_frequency.entry(*timeframe).or_insert(0) += 1;
        }

        for (timeframe, count) in timeframe_frequency {
            if count > 1 {
                report.add_warning(format!(
                    "Timeframe {:?} is used by multiple categories (may be redundant)",
                    timeframe
                ));
            }
        }

        // Verificar orden lógico de timeframes
        let current_tf = mapping.get(&TimeframeCategory::Current);
        let medium_tf = mapping.get(&TimeframeCategory::Medium);
        let high_tf = mapping.get(&TimeframeCategory::High);

        if let (Some(current), Some(medium)) = (current_tf, medium_tf) {
            if !self.is_timeframe_higher(medium, current) {
                report.add_error(format!(
                    "Medium timeframe ({:?}) should be higher than Current timeframe ({:?})",
                    medium, current
                ));
            }
        }

        if let (Some(medium), Some(high)) = (medium_tf, high_tf) {
            if !self.is_timeframe_higher(high, medium) {
                report.add_error(format!(
                    "High timeframe ({:?}) should be higher than Medium timeframe ({:?})",
                    high, medium
                ));
            }
        }
    }

    /// Helper: Verifica si timeframe1 es superior a timeframe2
    fn is_timeframe_higher(&self, tf1: &darwinx_core::TimeFrame, tf2: &darwinx_core::TimeFrame) -> bool {
        use darwinx_core::TimeFrame::*;
        
        let order = [M1, M5, M15, M30, H1, H4, D1, W1, MN1];
        let pos1 = order.iter().position(|&x| x == *tf1).unwrap_or(0);
        let pos2 = order.iter().position(|&x| x == *tf2).unwrap_or(0);
        
        pos1 > pos2
    }

    /// ✨ NEW: Warnings específicos para multi-timeframe
    fn check_multi_timeframe_warnings(&self, strategy: &StrategyAST, report: &mut ValidationReport) {
        let used_categories = strategy.used_timeframe_categories();

        // Warning: Solo usa Current timeframe pero podría beneficiarse de contexto
        if used_categories.len() == 1 && used_categories.contains(&TimeframeCategory::Current) {
            let indicator_count = strategy.all_indicators().len();
            if indicator_count >= 3 {
                report.add_warning(
                    "Strategy uses multiple indicators but only Current timeframe. Consider adding higher timeframe context for trend confirmation.".to_string()
                );
            }
        }

        // Warning: Usa solo High timeframe (puede ser muy lento para señales)
        if used_categories.len() == 1 && used_categories.contains(&TimeframeCategory::High) {
            report.add_warning(
                "Strategy uses only High timeframe. Consider adding Current timeframe indicators for more frequent signals.".to_string()
            );
        }

        // Warning: Demasiados indicadores en timeframes altos
        let stats = strategy.indicator_stats_by_timeframe();
        if let Some(&high_count) = stats.get(&TimeframeCategory::High) {
            if high_count > 2 {
                report.add_warning(format!(
                    "Too many indicators ({}) in High timeframe. High timeframe should typically be used for trend context only.",
                    high_count
                ));
            }
        }

        // Info: Configuración recomendada detectada
        if used_categories.contains(&TimeframeCategory::Current) 
           && used_categories.contains(&TimeframeCategory::Medium) 
           && strategy.complexity() <= 6 {
            report.add_info("✅ Good multi-timeframe configuration detected: Current + Medium timeframes with reasonable complexity.".to_string());
        }
    }

    /// Validar todos los indicadores
    fn validate_all_indicators(&self, strategy: &StrategyAST, report: &mut ValidationReport) {
        // Validar condiciones de entrada
        self.validate_conditions(&strategy.entry_rules.conditions, "entrada", report);
        
        // Validar condiciones de salida
        self.validate_conditions(&strategy.exit_rules.conditions, "salida", report);
    }

    fn count_unique_indicators(&self, strategy: &StrategyAST) -> usize {
        let mut indicators = HashSet::new();

        for indicator in strategy.all_indicators() {
            // Incluir timeframe en la clave única
            let key = format!("{}({:?})@{:?}", 
                indicator.name(), 
                indicator.params(), 
                indicator.timeframe_category()
            );
            indicators.insert(key);
        }

        indicators.len()
    }

    fn validate_conditions(
        &self,
        conditions: &[Condition],
        rule_type: &str,
        report: &mut ValidationReport,
    ) {
        for (idx, condition) in conditions.iter().enumerate() {
            let location = format!("{} #{}", rule_type, idx + 1);
            self.validate_condition(condition, &location, report);
        }
    }

    fn validate_condition(&self, condition: &Condition, location: &str, report: &mut ValidationReport) {
        // Validar indicador principal
        self.validate_indicator(&condition.indicator, location, report);

        // Validar valor si es otro indicador
        if let ConditionValue::Indicator(ind) = &condition.value {
            let loc = format!("{} (valor de comparación)", location);
            self.validate_indicator(ind, &loc, report);
            
            // ✨ NEW: Validación específica para comparaciones entre indicadores multi-TF
            self.validate_cross_timeframe_comparison(&condition.indicator, ind, location, report);
        }
    }

    /// ✨ NEW: Validar comparaciones entre indicadores de diferentes timeframes
    fn validate_cross_timeframe_comparison(
        &self, 
        ind1: &IndicatorType, 
        ind2: &IndicatorType, 
        location: &str, 
        report: &mut ValidationReport
    ) {
        if ind1.timeframe_category() != ind2.timeframe_category() {
            report.add_warning(format!(
                "{}: Comparing indicators from different timeframes ({} vs {}). Ensure this is intentional for your strategy logic.",
                location,
                ind1.timeframe_category().display_name(),
                ind2.timeframe_category().display_name()
            ));
        }

        // Warning para comparaciones que pueden no tener sentido
        if ind1.timeframe_category() == TimeframeCategory::High 
           && ind2.timeframe_category() == TimeframeCategory::Current {
            report.add_warning(format!(
                "{}: Comparing High timeframe indicator with Current timeframe. This may cause lookahead bias in backtesting.",
                location
            ));
        }
    }

    /// 🎯 Validación 100% dinámica usando metadata del registry
    /// 
    /// NO tiene casos específicos por indicador.
    /// Todo se valida usando la metadata del registry.
    fn validate_indicator(&self, indicator: &IndicatorType, location: &str, report: &mut ValidationReport) {
        let name = indicator.name();
        
        // Verificar que el indicador existe en el registry
        let metadata = match registry::get(name) {
            Some(meta) => meta,
            None => {
                report.add_error(format!(
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
            report.add_error(format!(
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
                report.add_error(format!(
                    "{}: '{}' parámetro {} ('{}') = {:.2} está por debajo del mínimo {:.2}",
                    location, name, idx + 1, param_def.name, param_value, param_def.min
                ));
            }
            
            // Verificar máximo
            if param_value > param_def.max {
                report.add_error(format!(
                    "{}: '{}' parámetro {} ('{}') = {:.2} está por encima del máximo {:.2}",
                    location, name, idx + 1, param_def.name, param_value, param_def.max
                ));
            }
        }

        // ✨ NEW: Info sobre timeframe del indicador
        report.add_info(format!(
            "{}: {} @ {} timeframe",
            location,
            indicator.display(),
            indicator.timeframe_category().display_name()
        ));
    }

    /// ✨ NEW: Análisis de calidad de la estrategia
    fn analyze_strategy_quality(&self, strategy: &StrategyAST, report: &mut ValidationReport) {
        // Verificar duplicados
        let duplicate_warnings = self.check_duplicates(strategy);
        for warning in duplicate_warnings {
            report.add_warning(warning);
        }

        // Verificar complejidad lógica
        let complexity_warnings = self.check_complexity(strategy);
        for warning in complexity_warnings {
            report.add_warning(warning);
        }

        // ✨ NEW: Análisis específico multi-timeframe
        self.analyze_multi_timeframe_balance(strategy, report);
    }

    /// ✨ NEW: Analizar balance de la configuración multi-timeframe
    fn analyze_multi_timeframe_balance(&self, strategy: &StrategyAST, report: &mut ValidationReport) {
        let stats = strategy.indicator_stats_by_timeframe();
        let total_indicators = strategy.all_indicators().len();

        if strategy.is_multi_timeframe() {
            // Analizar distribución de indicadores
            let current_ratio = stats.get(&TimeframeCategory::Current).unwrap_or(&0)
                as f64 / total_indicators as f64;
            let medium_ratio = stats.get(&TimeframeCategory::Medium).unwrap_or(&0)
                as f64 / total_indicators as f64;
            let high_ratio = stats.get(&TimeframeCategory::High).unwrap_or(&0)
                as f64 / total_indicators as f64;

            // Recomendaciones basadas en distribución
            if current_ratio < 0.3 && total_indicators > 2 {
                report.add_warning(
                    "Low proportion of Current timeframe indicators. Consider adding more for signal generation.".to_string()
                );
            }

            if high_ratio > 0.4 && total_indicators > 3 {
                report.add_warning(
                    "High proportion of High timeframe indicators. Consider using fewer for trend context only.".to_string()
                );
            }

            if medium_ratio > 0.6 && total_indicators > 3 {
                report.add_info(
                    "Good balance: Medium timeframe indicators provide good intermediate context.".to_string()
                );
            }
        }
    }

    /// Verifica duplicados innecesarios (warning, no error)
    pub fn check_duplicates(&self, strategy: &StrategyAST) -> Vec<String> {
        let mut warnings = Vec::new();
        let mut seen = HashMap::new();

        // Contar ocurrencias de cada combinación indicador+params+timeframe
        for indicator in strategy.all_indicators() {
            let key = format!("{}({:?})@{:?}", 
                indicator.name(), 
                indicator.params(),
                indicator.timeframe_category()
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

        // ✨ NEW: Análisis específico multi-timeframe
        if strategy.is_multi_timeframe() && strategy.complexity() > 8 {
            warnings.push(
                "Multi-timeframe strategy with high complexity. Consider simplifying for better performance.".to_string()
            );
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
    fn test_multi_timeframe_validation() {
        let strategy = StrategyBuilder::new("Multi-TF Test".to_string(), TimeFrame::M5)
            .add_entry_condition_with_timeframe(
                ConditionBuilder::above("rsi", vec![14.0], 50.0),
                TimeframeCategory::Current
            )
            .add_entry_condition_with_timeframe(
                ConditionBuilder::above("ema", vec![200.0], 100.0),
                TimeframeCategory::Medium
            )
            .add_exit_condition_with_timeframe(
                ConditionBuilder::below("rsi", vec![14.0], 50.0),
                TimeframeCategory::Current
            )
            .build();

        let validator = StrategyValidator::new(StrategyConstraints::default());
        let report = validator.validate_detailed(&strategy);
        
        assert!(report.is_valid());
        assert!(report.info.iter().any(|i| i.contains("Multi-timeframe strategy detected")));
    }

    #[test]
    fn test_timeframe_consistency_validation() {
        // This test would require a way to create invalid timeframe mappings
        // For now, we test the basic structure
        let strategy = StrategyBuilder::new("Test".to_string(), TimeFrame::H1)
            .add_entry_condition_with_timeframe(
                ConditionBuilder::above("rsi", vec![14.0], 50.0),
                TimeframeCategory::Current
            )
            .add_exit_condition_with_timeframe(
                ConditionBuilder::below("rsi", vec![14.0], 50.0),
                TimeframeCategory::Current
            )
            .build();

        let validator = StrategyValidator::new(StrategyConstraints::default());
        let report = validator.validate_detailed(&strategy);
        
        assert!(report.is_valid());
    }

    #[test]
    fn test_cross_timeframe_comparison_warning() {
        let mut strategy = StrategyBuilder::new("Cross-TF Test".to_string(), TimeFrame::M5);
        
        // Add condition comparing indicators from different timeframes
        let condition = ConditionBuilder::indicator_above_multi_tf(
            "ema", vec![50.0], TimeframeCategory::Current,
            "sma", vec![200.0], TimeframeCategory::High
        );
        
        strategy = strategy.add_entry_condition_with_timeframe(condition, TimeframeCategory::Current)
            .add_exit_condition_with_timeframe(
                ConditionBuilder::below("rsi", vec![14.0], 30.0),
                TimeframeCategory::Current
            );

        let built_strategy = strategy.build();
        let validator = StrategyValidator::new(StrategyConstraints::default());
        let report = validator.validate_detailed(&built_strategy);
        
        assert!(report.is_valid());
        // Should have warning about cross-timeframe comparison
        assert!(report.warnings.iter().any(|w| w.contains("different timeframes")));
    }

    #[test]
    fn test_valid_multi_tf_strategy() {
        let strategy = StrategyBuilder::golden_cross_multi_tf(
            "Valid Golden Cross".to_string(),
            TimeFrame::H1,
            50, 200, 14
        ).build();

        let validator = StrategyValidator::new(StrategyConstraints::default());
        let result = validator.validate(&strategy);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_too_many_timeframes() {
        let mut strategy = StrategyBuilder::new("Too Many TF".to_string(), TimeFrame::M5);
        
        // Add indicators from all timeframes to exceed limit
        strategy = strategy
            .add_entry_condition_with_timeframe(
                ConditionBuilder::above("rsi", vec![14.0], 50.0),
                TimeframeCategory::Current
            )
            .add_entry_condition_with_timeframe(
                ConditionBuilder::above("ema", vec![50.0], 100.0),
                TimeframeCategory::Medium
            )
            .add_entry_condition_with_timeframe(
                ConditionBuilder::above("sma", vec![200.0], 100.0),
                TimeframeCategory::High
            )
            .add_exit_condition_with_timeframe(
                ConditionBuilder::below("rsi", vec![14.0], 30.0),
                TimeframeCategory::Current
            );

        let built_strategy = strategy.build();
        
        // Test with strict constraints that limit timeframes
        let strict_constraints = StrategyConstraints::new(10, 5, 2); // max 2 timeframes
        let validator = StrategyValidator::new(strict_constraints);
        let result = validator.validate(&built_strategy);
        
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("timeframes") && e.contains("excede")));
    }

    #[test]
    fn test_validation_report_structure() {
        let strategy = StrategyBuilder::mean_reversion_multi_tf(
            "Report Test".to_string(),
            TimeFrame::M15,
            14, 50, 20
        ).build();

        let validator = StrategyValidator::new(StrategyConstraints::default());
        let report = validator.validate_detailed(&strategy);
        
        // Should have info messages
        assert!(!report.info.is_empty());
        assert!(report.info.iter().any(|i| i.contains("Strategy:")));
        assert!(report.info.iter().any(|i| i.contains("Complexity:")));
        
        // Should be valid
        assert!(report.is_valid());
        assert_eq!(report.to_result(), Ok(()));
    }

    #[test]
    fn test_backward_compatibility() {
        // Test que el método legacy validate() sigue funcionando
        let strategy = StrategyBuilder::new("Legacy Test".to_string(), TimeFrame::H1)
            .add_entry_condition(ConditionBuilder::above("rsi", vec![14.0], 70.0))
            .add_exit_condition(ConditionBuilder::below("rsi", vec![14.0], 30.0))
            .build();

        let validator = StrategyValidator::new(StrategyConstraints::default());
        let result = validator.validate(&strategy);
        
        assert!(result.is_ok());
    }
}
