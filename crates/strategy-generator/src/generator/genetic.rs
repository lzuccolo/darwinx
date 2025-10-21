//! Algoritmo genético para evolución de estrategias

use crate::ast::nodes::*;
use crate::generator::random::RandomGenerator;
use rand::Rng;

/// Configuración del algoritmo genético
pub struct GeneticConfig {
    pub population_size: usize,
    pub generations: usize,
    pub mutation_rate: f64,
    pub elite_size: usize,
}

impl Default for GeneticConfig {
    fn default() -> Self {
        Self {
            population_size: 100,
            generations: 50,
            mutation_rate: 0.1,
            elite_size: 10,
        }
    }
}

/// Generador genético de estrategias
pub struct GeneticGenerator {
    config: GeneticConfig,
    random_gen: RandomGenerator,
}

impl GeneticGenerator {
    pub fn new(config: GeneticConfig) -> Self {
        Self {
            config,
            random_gen: RandomGenerator::new(),
        }
    }

    /// Genera población inicial
    pub fn generate_population(&self, count: usize) -> Vec<StrategyAST> {
        self.random_gen.generate_batch(count)
    }

    /// Cruza dos estrategias (crossover)
    pub fn crossover(&self, parent1: &StrategyAST, parent2: &StrategyAST) -> StrategyAST {
        let mut rng = rand::thread_rng();
        let mut child = StrategyAST::new(
            format!("{}_{}", parent1.name, parent2.name),
            parent1.timeframe,
        );

        // Crossover de condiciones de entrada
        let p1_entry_len = parent1.entry_rules.conditions.len();
        let p2_entry_len = parent2.entry_rules.conditions.len();
        
        if p1_entry_len > 0 && p2_entry_len > 0 {
            let split = rng.gen_range(0..=p1_entry_len.min(p2_entry_len));
            child.entry_rules.conditions.extend_from_slice(&parent1.entry_rules.conditions[..split]);
            if split < p2_entry_len {
                child.entry_rules.conditions.extend_from_slice(&parent2.entry_rules.conditions[split..]);
            }
        }

        // Crossover de condiciones de salida (tomar del mejor padre)
        if rng.gen_bool(0.5) {
            child.exit_rules = parent1.exit_rules.clone();
        } else {
            child.exit_rules = parent2.exit_rules.clone();
        }

        child
    }

    /// Muta una estrategia
    pub fn mutate(&self, strategy: &mut StrategyAST) {
        let mut rng = rand::thread_rng();

        if rng.gen_bool(self.config.mutation_rate) {
            // Mutar una condición de entrada
            if !strategy.entry_rules.conditions.is_empty() {
                let idx = rng.gen_range(0..strategy.entry_rules.conditions.len());
                strategy.entry_rules.conditions[idx] = self.random_gen_condition();
            }
        }

        if rng.gen_bool(self.config.mutation_rate) {
            // Mutar una condición de salida
            if !strategy.exit_rules.conditions.is_empty() {
                let idx = rng.gen_range(0..strategy.exit_rules.conditions.len());
                strategy.exit_rules.conditions[idx] = self.random_gen_condition();
            }
        }

        if rng.gen_bool(self.config.mutation_rate * 0.5) {
            // Mutar operador lógico
            strategy.entry_rules.operator = if rng.gen_bool(0.5) {
                LogicalOperator::And
            } else {
                LogicalOperator::Or
            };
        }
    }

    /// Selección por torneo
    pub fn tournament_selection<F>(
        &self,
        population: &[StrategyAST],
        fitness_fn: &F,
        tournament_size: usize,
    ) -> StrategyAST
    where
        F: Fn(&StrategyAST) -> f64,
    {
        let mut rng = rand::thread_rng();
        let mut best: Option<(&StrategyAST, f64)> = None;

        for _ in 0..tournament_size {
            let idx = rng.gen_range(0..population.len());
            let candidate = &population[idx];
            let fitness = fitness_fn(candidate);

            if best.is_none() || fitness > best.unwrap().1 {
                best = Some((candidate, fitness));
            }
        }

        best.unwrap().0.clone()
    }

    fn random_gen_condition(&self) -> Condition {
        let mut rng = rand::thread_rng();
        
        Condition {
            indicator: match rng.gen_range(0..3) {
                0 => IndicatorType::Rsi { period: rng.gen_range(7..=21) },
                1 => IndicatorType::Sma { period: rng.gen_range(10..=50) },
                _ => IndicatorType::Ema { period: rng.gen_range(10..=50) },
            },
            comparison: match rng.gen_range(0..3) {
                0 => Comparison::GreaterThan,
                1 => Comparison::LessThan,
                _ => Comparison::CrossesAbove,
            },
            value: ConditionValue::Number(rng.gen_range(20.0..80.0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::builder::*;
    use darwinx_core::TimeFrame;

    #[test]
    fn test_generate_population() {
        let generator = GeneticGenerator::new(GeneticConfig::default());
        let population = generator.generate_population(10);
        
        assert_eq!(population.len(), 10);
    }

    #[test]
    fn test_crossover() {
        let parent1 = StrategyBuilder::new("P1".to_string(), TimeFrame::H1)
            .add_entry_condition(ConditionBuilder::rsi_above(14, 70.0))
            .add_exit_condition(ConditionBuilder::rsi_below(14, 30.0))
            .build();

        let parent2 = StrategyBuilder::new("P2".to_string(), TimeFrame::H1)
            .add_entry_condition(ConditionBuilder::sma_cross_above(10, 30))
            .add_exit_condition(ConditionBuilder::sma_cross_below(10, 30))
            .build();

        let generator = GeneticGenerator::new(GeneticConfig::default());
        let child = generator.crossover(&parent1, &parent2);

        assert!(!child.entry_rules.conditions.is_empty());
        assert!(!child.exit_rules.conditions.is_empty());
    }

    #[test]
    fn test_mutation() {
        let mut strategy = StrategyBuilder::new("Test".to_string(), TimeFrame::H1)
            .add_entry_condition(ConditionBuilder::rsi_above(14, 70.0))
            .add_exit_condition(ConditionBuilder::rsi_below(14, 30.0))
            .build();

        let original_complexity = strategy.complexity();
        
        let generator = GeneticGenerator::new(GeneticConfig {
            mutation_rate: 1.0, // 100% para testing
            ..Default::default()
        });

        generator.mutate(&mut strategy);

        // La estrategia debe seguir siendo válida
        assert!(!strategy.entry_rules.conditions.is_empty());
        assert!(!strategy.exit_rules.conditions.is_empty());
    }
}