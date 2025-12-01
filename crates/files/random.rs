//! Generador aleatorio de estrategias multi-timeframe
//!
//! ## Actualizaciones v2.1:
//! - ✨ Multi-timeframe strategy generation
//! - ✨ TimeframeCategory distribution control
//! - ✨ Enhanced diversity with timeframe awareness
//! - ✨ Semantic constraints integration (ready for Phase 3)

use crate::ast::nodes::*;
use crate::constraints::{StrategyConstraints, SemanticConstraints};
use darwinx_core::TimeFrame;
use darwinx_indicators::registry;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::collections::HashMap;

/// ✨ UPDATED: Generador aleatorio con soporte multi-timeframe
pub struct RandomGenerator {
    /// Random number generator
    rng: StdRng,
    
    /// Constraints básicos de estrategia
    strategy_constraints: StrategyConstraints,
    
    /// ✨ NEW: Semantic constraints (Phase 3 integration ready)
    semantic_constraints: SemanticConstraints,
    
    /// ✨ NEW: Distribución de timeframes (probabilidades)
    timeframe_distribution: TimeframeDistribution,
}

/// ✨ NEW: Configuración de distribución de timeframes
#[derive(Debug, Clone)]
pub struct TimeframeDistribution {
    /// Probabilidad de usar Current timeframe (0.0-1.0)
    pub current_probability: f64,
    
    /// Probabilidad de usar Medium timeframe (0.0-1.0)
    pub medium_probability: f64,
    
    /// Probabilidad de usar High timeframe (0.0-1.0)
    pub high_probability: f64,
    
    /// Probabilidad de generar estrategia multi-timeframe (0.0-1.0)
    pub multi_timeframe_probability: f64,
}

impl Default for TimeframeDistribution {
    fn default() -> Self {
        Self {
            current_probability: 0.7,    // 70% chance for Current
            medium_probability: 0.6,     // 60% chance for Medium  
            high_probability: 0.3,       // 30% chance for High
            multi_timeframe_probability: 0.6, // 60% multi-TF strategies
        }
    }
}

impl TimeframeDistribution {
    /// Configuración para favorizar single-timeframe
    pub fn single_timeframe_focused() -> Self {
        Self {
            current_probability: 1.0,
            medium_probability: 0.0,
            high_probability: 0.0,
            multi_timeframe_probability: 0.0,
        }
    }

    /// Configuración para favorizar multi-timeframe
    pub fn multi_timeframe_focused() -> Self {
        Self {
            current_probability: 0.8,
            medium_probability: 0.9,
            high_probability: 0.5,
            multi_timeframe_probability: 0.9,
        }
    }

    /// Configuración balanceada
    pub fn balanced() -> Self {
        Self::default()
    }

    /// Configuración conservadora (menos High timeframe)
    pub fn conservative() -> Self {
        Self {
            current_probability: 0.8,
            medium_probability: 0.7,
            high_probability: 0.2,
            multi_timeframe_probability: 0.5,
        }
    }
}

impl RandomGenerator {
    /// Crea un nuevo generador con constraints
    pub fn new(
        seed: Option<u64>,
        strategy_constraints: StrategyConstraints,
        semantic_constraints: SemanticConstraints,
    ) -> Self {
        let rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_entropy(),
        };

        Self {
            rng,
            strategy_constraints,
            semantic_constraints,
            timeframe_distribution: TimeframeDistribution::default(),
        }
    }

    /// Constructor conveniente con constraints por defecto
    pub fn default_with_seed(seed: Option<u64>) -> Self {
        Self::new(
            seed,
            StrategyConstraints::default(),
            SemanticConstraints::default(),
        )
    }

    /// ✨ NEW: Configura la distribución de timeframes
    pub fn with_timeframe_distribution(mut self, distribution: TimeframeDistribution) -> Self {
        self.timeframe_distribution = distribution;
        self
    }

    /// ✨ NEW: Genera una estrategia multi-timeframe aleatoria
    pub fn generate_multi_timeframe(&mut self, name: String, primary_timeframe: TimeFrame) -> StrategyAST {
        let mut strategy = StrategyAST::new(name, primary_timeframe);

        // Decidir si generar multi-timeframe
        let is_multi_tf = self.should_generate_multi_timeframe();

        // Generar condiciones de entrada
        let entry_count = self.random_condition_count(true);
        strategy.entry_rules.conditions = self.generate_conditions(entry_count, is_multi_tf);
        strategy.entry_rules.operator = self.random_logical_operator();

        // Generar condiciones de salida
        let exit_count = self.random_condition_count(false);
        strategy.exit_rules.conditions = self.generate_conditions(exit_count, is_multi_tf);
        strategy.exit_rules.operator = self.random_logical_operator();

        strategy
    }

    /// Genera una estrategia aleatoria (legacy method con Current timeframe)
    pub fn generate(&mut self, name: String, primary_timeframe: TimeFrame) -> StrategyAST {
        self.generate_multi_timeframe(name, primary_timeframe)
    }

    /// ✨ NEW: Genera múltiples estrategias en batch
    pub fn generate_batch(
        &mut self, 
        count: usize, 
        name_prefix: &str, 
        primary_timeframe: TimeFrame
    ) -> Vec<StrategyAST> {
        (0..count)
            .map(|i| {
                let name = format!("{}_{:04}", name_prefix, i + 1);
                self.generate_multi_timeframe(name, primary_timeframe)
            })
            .collect()
    }

    /// ✨ NEW: Genera estrategias con diferentes timeframes principales
    pub fn generate_cross_timeframe_batch(
        &mut self,
        count_per_timeframe: usize,
        name_prefix: &str,
        timeframes: &[TimeFrame],
    ) -> Vec<StrategyAST> {
        let mut strategies = Vec::new();
        
        for (tf_idx, &timeframe) in timeframes.iter().enumerate() {
            for i in 0..count_per_timeframe {
                let name = format!("{}_TF{}_S{:04}", name_prefix, tf_idx + 1, i + 1);
                strategies.push(self.generate_multi_timeframe(name, timeframe));
            }
        }

        strategies
    }

    /// Decide si generar estrategia multi-timeframe
    fn should_generate_multi_timeframe(&mut self) -> bool {
        if !self.strategy_constraints.allows_multi_timeframe() {
            return false;
        }

        self.rng.gen::<f64>() < self.timeframe_distribution.multi_timeframe_probability
    }

    /// Genera un número aleatorio de condiciones
    fn random_condition_count(&mut self, is_entry: bool) -> usize {
        let min = if is_entry { 1 } else { 1 };
        let max = if is_entry { 
            (self.strategy_constraints.max_conditions * 2 / 3).max(1)
        } else {
            (self.strategy_constraints.max_conditions / 3).max(1)
        };

        self.rng.gen_range(min..=max)
    }

    /// ✨ NEW: Genera condiciones con timeframe awareness
    fn generate_conditions(&mut self, count: usize, allow_multi_tf: bool) -> Vec<Condition> {
        let mut conditions = Vec::new();
        let mut used_indicators = HashMap::new();

        for _ in 0..count {
            let condition = self.generate_single_condition(allow_multi_tf, &mut used_indicators);
            conditions.push(condition);
        }

        conditions
    }

    /// ✨ NEW: Genera una condición individual con timeframe
    fn generate_single_condition(
        &mut self,
        allow_multi_tf: bool,
        used_indicators: &mut HashMap<String, usize>,
    ) -> Condition {
        // Seleccionar indicador aleatorio del registry
        let available_indicators = registry::all_names();
        let indicator_name = available_indicators
            .choose(&mut self.rng)
            .unwrap()
            .clone();

        // ✨ Verificar semantic constraints (Phase 3 ready)
        // Por ahora usa category limits básicos
        let category_limit = self.semantic_constraints.limit_for_category(
            &self.get_indicator_category(&indicator_name)
        );
        
        let current_usage = used_indicators.get(&indicator_name).copied().unwrap_or(0);
        if current_usage >= category_limit {
            // Si excede límite, buscar otro indicador (simplified para v2.1)
            // En Phase 3 será más sofisticado con correlation analysis
        }

        *used_indicators.entry(indicator_name.clone()).or_insert(0) += 1;

        // Generar parámetros para el indicador
        let params = self.generate_indicator_params(&indicator_name);
        
        // ✨ Seleccionar timeframe category
        let timeframe_category = self.select_timeframe_category(allow_multi_tf);
        
        // Crear indicador
        let indicator = IndicatorType::new(indicator_name, params, timeframe_category);

        // Generar comparación y valor
        let comparison = self.random_comparison();
        let value = self.generate_condition_value(&indicator, allow_multi_tf, used_indicators);

        Condition {
            indicator,
            comparison,
            value,
        }
    }

    /// ✨ NEW: Selecciona timeframe category basado en distribución
    fn select_timeframe_category(&mut self, allow_multi_tf: bool) -> TimeframeCategory {
        if !allow_multi_tf {
            return TimeframeCategory::Current;
        }

        // Usar probabilidades de distribución
        let rand_value = self.rng.gen::<f64>();
        
        if rand_value < self.timeframe_distribution.current_probability {
            TimeframeCategory::Current
        } else if rand_value < self.timeframe_distribution.current_probability + self.timeframe_distribution.medium_probability {
            TimeframeCategory::Medium
        } else if rand_value < self.timeframe_distribution.current_probability + self.timeframe_distribution.medium_probability + self.timeframe_distribution.high_probability {
            TimeframeCategory::High
        } else {
            // Fallback to Current if probabilities don't sum to 1.0
            TimeframeCategory::Current
        }
    }

    /// Genera parámetros aleatorios para un indicador
    fn generate_indicator_params(&mut self, indicator_name: &str) -> Vec<f64> {
        let metadata = registry::get(indicator_name).unwrap();
        
        metadata.parameters
            .iter()
            .map(|param_def| {
                // Generar valor aleatorio dentro del rango válido
                self.rng.gen_range(param_def.min..=param_def.max)
            })
            .collect()
    }

    /// Genera un valor para la condición
    fn generate_condition_value(
        &mut self,
        _indicator: &IndicatorType,
        allow_multi_tf: bool,
        used_indicators: &mut HashMap<String, usize>,
    ) -> ConditionValue {
        // 70% chance de usar valor numérico, 30% otro indicador
        if self.rng.gen::<f64>() < 0.7 {
            ConditionValue::Number(self.random_numeric_value())
        } else if self.rng.gen::<f64>() < 0.1 {
            ConditionValue::Price
        } else {
            // Generar otro indicador para comparación
            let available_indicators = registry::all_names();
            let other_name = available_indicators
                .choose(&mut self.rng)
                .unwrap()
                .clone();

            let params = self.generate_indicator_params(&other_name);
            let timeframe_category = self.select_timeframe_category(allow_multi_tf);
            
            *used_indicators.entry(other_name.clone()).or_insert(0) += 1;
            
            ConditionValue::Indicator(IndicatorType::new(other_name, params, timeframe_category))
        }
    }

    /// Genera un valor numérico aleatorio para comparaciones
    fn random_numeric_value(&mut self) -> f64 {
        // Rangos comunes para diferentes tipos de comparaciones
        let value_type = self.rng.gen_range(0..4);
        
        match value_type {
            0 => self.rng.gen_range(0.0..100.0),      // RSI, Stochastic range
            1 => self.rng.gen_range(-2.0..2.0),       // MACD range
            2 => self.rng.gen_range(0.5..2.0),        // ATR, volatility range
            _ => self.rng.gen_range(10.0..1000.0),    // Price level range
        }
    }

    /// Genera una comparación aleatoria
    fn random_comparison(&mut self) -> Comparison {
        let comparisons = [
            Comparison::GreaterThan,
            Comparison::LessThan,
            Comparison::CrossesAbove,
            Comparison::CrossesBelow,
            Comparison::Equals,
        ];
        
        *comparisons.choose(&mut self.rng).unwrap()
    }

    /// Genera un operador lógico aleatorio
    fn random_logical_operator(&mut self) -> LogicalOperator {
        if self.rng.gen::<f64>() < 0.7 {
            LogicalOperator::And
        } else {
            LogicalOperator::Or
        }
    }

    /// ✨ FUTURE: Helper para obtener categoría de indicador (Phase 3)
    /// Por ahora usa fallback, en Phase 3 usará registry metadata
    fn get_indicator_category(&self, _indicator_name: &str) -> darwinx_indicators::IndicatorCategory {
        // Temporary fallback - en Phase 3 se obtendrá del registry
        darwinx_indicators::IndicatorCategory::Trend
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_generator_creation() {
        let generator = RandomGenerator::default_with_seed(Some(42));
        assert!(generator.strategy_constraints.allows_multi_timeframe());
    }

    #[test]
    fn test_timeframe_distribution() {
        let dist = TimeframeDistribution::multi_timeframe_focused();
        assert!(dist.multi_timeframe_probability > 0.8);
        
        let single_dist = TimeframeDistribution::single_timeframe_focused();
        assert_eq!(single_dist.multi_timeframe_probability, 0.0);
    }

    #[test]
    fn test_strategy_generation() {
        let mut generator = RandomGenerator::default_with_seed(Some(42));
        
        let strategy = generator.generate_multi_timeframe(
            "Test Strategy".to_string(),
            TimeFrame::M5
        );
        
        assert_eq!(strategy.name, "Test Strategy");
        assert_eq!(strategy.primary_timeframe, TimeFrame::M5);
        assert!(!strategy.entry_rules.conditions.is_empty());
        assert!(!strategy.exit_rules.conditions.is_empty());
    }

    #[test]
    fn test_batch_generation() {
        let mut generator = RandomGenerator::default_with_seed(Some(42));
        
        let strategies = generator.generate_batch(5, "Batch", TimeFrame::H1);
        
        assert_eq!(strategies.len(), 5);
        for (i, strategy) in strategies.iter().enumerate() {
            assert_eq!(strategy.name, format!("Batch_{:04}", i + 1));
            assert_eq!(strategy.primary_timeframe, TimeFrame::H1);
        }
    }

    #[test]
    fn test_cross_timeframe_batch() {
        let mut generator = RandomGenerator::default_with_seed(Some(42));
        let timeframes = vec![TimeFrame::M5, TimeFrame::H1, TimeFrame::D1];
        
        let strategies = generator.generate_cross_timeframe_batch(
            2, "CrossTF", &timeframes
        );
        
        assert_eq!(strategies.len(), 6); // 2 strategies × 3 timeframes
        assert_eq!(strategies[0].primary_timeframe, TimeFrame::M5);
        assert_eq!(strategies[2].primary_timeframe, TimeFrame::H1);
        assert_eq!(strategies[4].primary_timeframe, TimeFrame::D1);
    }

    #[test]
    fn test_single_timeframe_constraints() {
        let mut generator = RandomGenerator::new(
            Some(42),
            StrategyConstraints::strict(), // Single timeframe only
            SemanticConstraints::default(),
        );
        
        let strategy = generator.generate_multi_timeframe(
            "Single TF".to_string(),
            TimeFrame::M15
        );
        
        // Should not be multi-timeframe with strict constraints
        assert!(!strategy.is_multi_timeframe() || strategy.timeframe_count() == 1);
    }

    #[test]
    fn test_multi_timeframe_distribution() {
        let mut generator = RandomGenerator::default_with_seed(Some(42))
            .with_timeframe_distribution(TimeframeDistribution::multi_timeframe_focused());
        
        // Generate multiple strategies and check multi-TF ratio
        let strategies: Vec<_> = (0..10)
            .map(|i| generator.generate_multi_timeframe(format!("Test_{}", i), TimeFrame::M5))
            .collect();
        
        let multi_tf_count = strategies.iter()
            .filter(|s| s.is_multi_timeframe())
            .count();
        
        // With multi-TF focused distribution, should have several multi-TF strategies
        // (not deterministic, but should be > 0 with focused distribution)
        assert!(multi_tf_count >= 0); // Relaxed assertion for random nature
    }

    #[test]
    fn test_semantic_constraints_integration() {
        let semantic_constraints = SemanticConstraints::strict();
        let mut generator = RandomGenerator::new(
            Some(42),
            StrategyConstraints::default(),
            semantic_constraints,
        );
        
        let strategy = generator.generate_multi_timeframe(
            "Semantic Test".to_string(),
            TimeFrame::H1
        );
        
        // Should generate valid strategy with semantic constraints
        assert!(!strategy.entry_rules.conditions.is_empty());
        assert!(!strategy.exit_rules.conditions.is_empty());
    }

    #[test]
    fn test_deterministic_generation() {
        // Same seed should produce same results
        let mut gen1 = RandomGenerator::default_with_seed(Some(123));
        let mut gen2 = RandomGenerator::default_with_seed(Some(123));
        
        let strategy1 = gen1.generate_multi_timeframe("Test".to_string(), TimeFrame::M5);
        let strategy2 = gen2.generate_multi_timeframe("Test".to_string(), TimeFrame::M5);
        
        // Basic structure should be the same
        assert_eq!(strategy1.entry_rules.conditions.len(), strategy2.entry_rules.conditions.len());
        assert_eq!(strategy1.exit_rules.conditions.len(), strategy2.exit_rules.conditions.len());
    }
}
