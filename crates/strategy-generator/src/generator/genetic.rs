//! Algoritmo genético - 100% DINÁMICO usando registry

use crate::ast::nodes::*;
use crate::generator::random::RandomGenerator;
use darwinx_indicators::registry;
use rand::prelude::*;

/// Configuración del algoritmo genético
#[derive(Debug, Clone)]
pub struct GeneticConfig {
    pub population_size: usize,
    pub generations: usize,
    pub mutation_rate: f64,
    pub elite_size: usize,
    pub tournament_size: usize,
}

impl Default for GeneticConfig {
    fn default() -> Self {
        Self {
            population_size: 100,
            generations: 50,
            mutation_rate: 0.1,
            elite_size: 10,
            tournament_size: 3,
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

        // Crossover de condiciones de salida
        if rng.gen_bool(0.5) {
            child.exit_rules = parent1.exit_rules.clone();
        } else {
            child.exit_rules = parent2.exit_rules.clone();
        }

        // Crossover de operadores lógicos
        child.entry_rules.operator = if rng.gen_bool(0.5) {
            parent1.entry_rules.operator
        } else {
            parent2.entry_rules.operator
        };

        child
    }

    /// Muta una estrategia
    /// 
    /// Tipos de mutación:
    /// 1. Reemplazar condición completa
    /// 2. Mutar parámetros de un indicador existente
    /// 3. Cambiar operador lógico
    /// 4. Cambiar comparador
    pub fn mutate(&self, strategy: &mut StrategyAST) {
        let mut rng = rand::thread_rng();

        // Mutación 1: Reemplazar condición de entrada
        if rng.gen_bool(self.config.mutation_rate) {
            if !strategy.entry_rules.conditions.is_empty() {
                let idx = rng.gen_range(0..strategy.entry_rules.conditions.len());
                strategy.entry_rules.conditions[idx] = self.random_condition(&mut rng);
            }
        }

        // Mutación 2: Reemplazar condición de salida
        if rng.gen_bool(self.config.mutation_rate) {
            if !strategy.exit_rules.conditions.is_empty() {
                let idx = rng.gen_range(0..strategy.exit_rules.conditions.len());
                strategy.exit_rules.conditions[idx] = self.random_condition(&mut rng);
            }
        }

        // Mutación 3: Cambiar operador lógico
        if rng.gen_bool(self.config.mutation_rate * 0.5) {
            strategy.entry_rules.operator = if rng.gen_bool(0.5) {
                LogicalOperator::And
            } else {
                LogicalOperator::Or
            };
        }

        // Mutación 4: Ajustar parámetros de indicadores existentes
        if rng.gen_bool(self.config.mutation_rate * 0.3) {
            self.mutate_parameters(strategy, &mut rng);
        }

        // Mutación 5: Cambiar comparador
        if rng.gen_bool(self.config.mutation_rate * 0.2) {
            self.mutate_comparison(strategy, &mut rng);
        }
    }

    /// Mutación de parámetros de indicadores existentes
    /// 
    /// Ajusta los parámetros dentro de rangos válidos según metadata
    fn mutate_parameters(&self, strategy: &mut StrategyAST, rng: &mut impl Rng) {
        let mut all_conditions: Vec<&mut Condition> = strategy
            .entry_rules
            .conditions
            .iter_mut()
            .chain(strategy.exit_rules.conditions.iter_mut())
            .collect();

        if all_conditions.is_empty() {
            return;
        }

        let idx = rng.gen_range(0..all_conditions.len());
        self.mutate_indicator_params(&mut all_conditions[idx].indicator, rng);
    }

    /// Muta los parámetros de un indicador usando metadata del registry
    fn mutate_indicator_params(&self, indicator: &mut IndicatorType, rng: &mut impl Rng) {
        let name = indicator.name();
        
        if let Some(metadata) = registry::get(name) {
            let new_params: Vec<f64> = metadata.parameters
                .iter()
                .zip(indicator.params().iter())
                .map(|(param_def, &current_val)| {
                    // Mutación pequeña aleatoria (±20% del rango)
                    let range = param_def.max - param_def.min;
                    let mutation_amount = range * 0.2;
                    let mutation = rng.gen_range(-mutation_amount..=mutation_amount);
                    let new_val = current_val + mutation;
                    
                    // Clamp al rango válido
                    new_val.clamp(param_def.min, param_def.max)
                })
                .collect();
            
            *indicator = IndicatorType::new(name.to_string(), new_params);
        }
    }

    /// Mutación de comparador
    fn mutate_comparison(&self, strategy: &mut StrategyAST, rng: &mut impl Rng) {
        let mut all_conditions: Vec<&mut Condition> = strategy
            .entry_rules
            .conditions
            .iter_mut()
            .chain(strategy.exit_rules.conditions.iter_mut())
            .collect();

        if all_conditions.is_empty() {
            return;
        }

        let idx = rng.gen_range(0..all_conditions.len());
        all_conditions[idx].comparison = self.random_comparison(rng);
    }

    /// Selección por torneo
    pub fn tournament_selection<F>(
        &self,
        population: &[StrategyAST],
        fitness_fn: &F,
    ) -> StrategyAST
    where
        F: Fn(&StrategyAST) -> f64,
    {
        let mut rng = rand::thread_rng();
        let mut best: Option<(&StrategyAST, f64)> = None;

        for _ in 0..self.config.tournament_size {
            let idx = rng.gen_range(0..population.len());
            let candidate = &population[idx];
            let fitness = fitness_fn(candidate);

            if best.is_none() || fitness > best.unwrap().1 {
                best = Some((candidate, fitness));
            }
        }

        best.unwrap().0.clone()
    }

    /// Evoluciona una población completa
    /// 
    /// Retorna la población final ordenada por fitness (mejor primero)
    pub fn evolve<F>(
        &self,
        initial_population: Vec<StrategyAST>,
        fitness_fn: F,
    ) -> Vec<StrategyAST>
    where
        F: Fn(&StrategyAST) -> f64,
    {
        let mut population = initial_population;
        let mut best_fitness = f64::NEG_INFINITY;
        let mut generations_without_improvement = 0;

        for generation in 0..self.config.generations {
            // Calcular fitness de toda la población
            let mut fitness_scores: Vec<(usize, f64)> = population
                .iter()
                .enumerate()
                .map(|(idx, strategy)| (idx, fitness_fn(strategy)))
                .collect();

            // Ordenar por fitness (mayor es mejor)
            fitness_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

            // Tracking de mejora
            let current_best = fitness_scores[0].1;
            if current_best > best_fitness {
                best_fitness = current_best;
                generations_without_improvement = 0;
            } else {
                generations_without_improvement += 1;
            }

            // Early stopping si no hay mejora
            if generations_without_improvement > 10 {
                break;
            }

            // Crear nueva generación
            let mut new_population = Vec::new();

            // Elitismo: mantener los mejores sin cambios
            for i in 0..self.config.elite_size.min(population.len()) {
                let idx = fitness_scores[i].0;
                new_population.push(population[idx].clone());
            }

            // Generar resto por crossover y mutación
            while new_population.len() < self.config.population_size {
                let parent1 = self.tournament_selection(&population, &fitness_fn);
                let parent2 = self.tournament_selection(&population, &fitness_fn);
                
                let mut child = self.crossover(&parent1, &parent2);
                self.mutate(&mut child);
                
                new_population.push(child);
            }

            population = new_population;
        }

        // Retornar población final ordenada por fitness
        let mut final_scores: Vec<(StrategyAST, f64)> = population
            .into_iter()
            .map(|s| {
                let fitness = fitness_fn(&s);
                (s, fitness)
            })
            .collect();

        final_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        final_scores.into_iter().map(|(s, _)| s).collect()
    }

    // ===== Métodos auxiliares 100% dinámicos =====

    /// Genera una condición aleatoria usando el registry
    fn random_condition(&self, rng: &mut impl Rng) -> Condition {
        let indicator = self.random_indicator(rng);
        let comparison = self.random_comparison(rng);
        let value = self.random_value(rng);

        Condition {
            indicator,
            comparison,
            value,
        }
    }

    /// Genera un indicador aleatorio del registry
    fn random_indicator(&self, rng: &mut impl Rng) -> IndicatorType {
        let available = registry::all_names();
        
        if available.is_empty() {
            // Fallback si no hay indicadores
            return IndicatorType::with_period("sma", 20);
        }
        
        let selected_name = available.choose(rng).unwrap();
        let meta = registry::get(selected_name)
            .expect("Indicator should be registered");
        
        // Generar parámetros aleatorios dentro de rangos válidos
        let params: Vec<f64> = meta.parameters
            .iter()
            .map(|param_def| rng.gen_range(param_def.min..=param_def.max))
            .collect();
        
        IndicatorType::new(selected_name.to_string(), params)
    }

    fn random_comparison(&self, rng: &mut impl Rng) -> Comparison {
        match rng.gen_range(0..5) {
            0 => Comparison::GreaterThan,
            1 => Comparison::LessThan,
            2 => Comparison::CrossesAbove,
            3 => Comparison::CrossesBelow,
            _ => Comparison::Equals,
        }
    }

    fn random_value(&self, rng: &mut impl Rng) -> ConditionValue {
        match rng.gen_range(0..3) {
            0 => ConditionValue::Number(rng.gen_range(20.0..80.0)),
            1 => ConditionValue::Price,
            _ => ConditionValue::Indicator(self.random_indicator(rng)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use darwinx_core::TimeFrame;

    #[test]
    fn test_generate_population() {
        let generator = GeneticGenerator::new(GeneticConfig::default());
        let population = generator.generate_population(10);
        
        assert_eq!(population.len(), 10);
        assert!(population.iter().all(|s| s.complexity() > 0));
    }

    #[test]
    fn test_crossover() {
        let generator = GeneticGenerator::new(GeneticConfig::default());
        let pop = generator.generate_population(2);
        
        let child = generator.crossover(&pop[0], &pop[1]);

        assert!(!child.entry_rules.conditions.is_empty() || !child.exit_rules.conditions.is_empty());
        assert!(child.name.contains(&pop[0].name) || child.name.contains(&pop[1].name));
    }

    #[test]
    fn test_mutation() {
        let generator = GeneticGenerator::new(GeneticConfig {
            mutation_rate: 1.0, // 100% para testing
            ..Default::default()
        });

        let mut pop = generator.generate_population(1);
        let original = pop[0].clone();

        // Mutar múltiples veces
        for _ in 0..10 {
            generator.mutate(&mut pop[0]);
        }

        // La estrategia debe seguir siendo válida
        assert!(!pop[0].entry_rules.conditions.is_empty());
        assert!(!pop[0].exit_rules.conditions.is_empty());
    }

    #[test]
    fn test_parameter_mutation() {
        let generator = GeneticGenerator::new(GeneticConfig {
            mutation_rate: 1.0,
            ..Default::default()
        });

        let mut pop = generator.generate_population(1);
        let original_indicators: Vec<_> = pop[0].entry_rules.conditions
            .iter()
            .map(|c| (c.indicator.name().to_string(), c.indicator.params().to_vec()))
            .collect();

        // Mutar parámetros múltiples veces
        for _ in 0..20 {
            generator.mutate(&mut pop[0]);
        }

        // Verificar que algunos parámetros cambiaron
        let new_indicators: Vec<_> = pop[0].entry_rules.conditions
            .iter()
            .map(|c| (c.indicator.name().to_string(), c.indicator.params().to_vec()))
            .collect();

        // Debe haber algún cambio
        assert!(original_indicators != new_indicators || 
                pop[0].entry_rules.conditions.len() != original_indicators.len());
    }

    #[test]
    fn test_tournament_selection() {
        let generator = GeneticGenerator::new(GeneticConfig::default());
        let population = generator.generate_population(10);

        // Fitness simple: preferir estrategias más complejas
        let fitness_fn = |s: &StrategyAST| s.complexity() as f64;

        // Seleccionar múltiples veces
        for _ in 0..5 {
            let selected = generator.tournament_selection(&population, &fitness_fn);
            assert!(selected.complexity() > 0);
        }
    }

    #[test]
    fn test_evolve() {
        let generator = GeneticGenerator::new(GeneticConfig {
            population_size: 20,
            generations: 5,
            mutation_rate: 0.2,
            elite_size: 2,
            tournament_size: 3,
        });

        let initial_pop = generator.generate_population(20);

        // Fitness: preferir estrategias con complejidad moderada
        let fitness_fn = |s: &StrategyAST| {
            let complexity = s.complexity() as f64;
            // Penalizar muy simple o muy complejo
            if complexity < 2.0 || complexity > 10.0 {
                complexity * 0.5
            } else {
                complexity * 2.0
            }
        };

        // Calcular fitness inicial
        let initial_fitness: Vec<_> = initial_pop.iter()
            .map(|s| fitness_fn(s))
            .collect();
        let avg_initial = initial_fitness.iter().sum::<f64>() / initial_fitness.len() as f64;

        // Evolucionar
        let evolved_pop = generator.evolve(initial_pop, fitness_fn);

        // Calcular fitness final
        let final_fitness: Vec<_> = evolved_pop.iter()
            .map(|s| fitness_fn(s))
            .collect();
        let avg_final = final_fitness.iter().sum::<f64>() / final_fitness.len() as f64;

        // La población final debe tener igual o mejor fitness promedio
        assert!(avg_final >= avg_initial * 0.9); // 90% del original como mínimo

        // La mejor estrategia final debe ser mejor que la inicial
        assert!(final_fitness[0] >= initial_fitness.iter().cloned().fold(f64::NEG_INFINITY, f64::max));

        // La población debe estar ordenada por fitness
        for i in 1..final_fitness.len() {
            assert!(final_fitness[i-1] >= final_fitness[i]);
        }
    }

    #[test]
    fn test_uses_registry_indicators() {
        let generator = GeneticGenerator::new(GeneticConfig::default());
        let population = generator.generate_population(50);

        // Verificar que usa indicadores del registry
        let available = registry::all_names();
        assert!(!available.is_empty(), "Registry should have indicators");

        // Contar indicadores únicos usados
        let mut indicator_names = std::collections::HashSet::new();
        for strategy in &population {
            for condition in &strategy.entry_rules.conditions {
                indicator_names.insert(condition.indicator.name().to_string());
            }
            for condition in &strategy.exit_rules.conditions {
                indicator_names.insert(condition.indicator.name().to_string());
            }
        }

        // Con 50 estrategias, debería haber variedad
        assert!(indicator_names.len() >= 3, 
            "Expected at least 3 different indicators, got: {:?}", 
            indicator_names);
    }

    #[test]
    fn test_elitism() {
        let generator = GeneticGenerator::new(GeneticConfig {
            population_size: 10,
            generations: 2,
            elite_size: 3,
            mutation_rate: 0.5,
            tournament_size: 2,
        });

        let initial_pop = generator.generate_population(10);
        let fitness_fn = |s: &StrategyAST| s.complexity() as f64;

        // Encontrar los 3 mejores iniciales
        let mut initial_scores: Vec<_> = initial_pop.iter()
            .map(|s| (s.name.clone(), fitness_fn(s)))
            .collect();
        initial_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Evolucionar
        let evolved_pop = generator.evolve(initial_pop, fitness_fn);

        // Los 3 mejores iniciales deberían estar en la población final
        // (o mejores versiones con misma complejidad)
        let final_scores: Vec<_> = evolved_pop.iter()
            .map(|s| fitness_fn(s))
            .collect();

        // El mejor final debe ser al menos tan bueno como el mejor inicial
        assert!(final_scores[0] >= initial_scores[0].1);
    }
}