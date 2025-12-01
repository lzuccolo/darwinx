//! Generador genético de estrategias multi-timeframe
//!
//! ## Características v2.1:
//! - ✨ Base structure para algoritmo genético
//! - ✨ Multi-timeframe aware fitness function
//! - ✨ Diversity-focused selection (semantic constraints ready)
//! 
//! ## Status: Base implementation - Full genetic algorithm in future phases

use crate::ast::nodes::*;
use crate::constraints::{StrategyConstraints, SemanticConstraints};
use crate::generator::random::RandomGenerator;
use darwinx_core::TimeFrame;
use rand::prelude::*;
use std::collections::HashMap;

/// ✨ Generador usando algoritmo genético para evolucionar estrategias
/// 
/// Implementación básica v2.1 - Algoritmo genético completo en fases futuras
pub struct GeneticGenerator {
    /// Generador aleatorio interno para población inicial
    random_generator: RandomGenerator,
    
    /// Constraints para estrategias
    strategy_constraints: StrategyConstraints,
    
    /// Semantic constraints para diversidad
    semantic_constraints: SemanticConstraints,
    
    /// Configuración del algoritmo genético
    config: GeneticConfig,
    
    /// Población actual
    population: Vec<Individual>,
}

/// Configuración del algoritmo genético
#[derive(Debug, Clone)]
pub struct GeneticConfig {
    /// Tamaño de la población
    pub population_size: usize,
    
    /// Número de generaciones
    pub max_generations: usize,
    
    /// Tasa de mutación (0.0-1.0)
    pub mutation_rate: f64,
    
    /// Tasa de crossover (0.0-1.0)
    pub crossover_rate: f64,
    
    /// Porcentaje de elite que sobrevive (0.0-1.0)
    pub elite_percentage: f64,
    
    /// ✨ Factor de diversidad en fitness (0.0-1.0)
    /// 0.0 = solo performance, 1.0 = solo diversidad
    pub diversity_factor: f64,
}

impl Default for GeneticConfig {
    fn default() -> Self {
        Self {
            population_size: 100,
            max_generations: 50,
            mutation_rate: 0.1,
            crossover_rate: 0.7,
            elite_percentage: 0.1,
            diversity_factor: 0.3, // Balance performance + diversidad
        }
    }
}

impl GeneticConfig {
    /// Configuración para diversidad máxima
    pub fn diversity_focused() -> Self {
        Self {
            diversity_factor: 0.7,
            mutation_rate: 0.15,
            ..Default::default()
        }
    }

    /// Configuración para performance máximo
    pub fn performance_focused() -> Self {
        Self {
            diversity_factor: 0.1,
            elite_percentage: 0.2,
            ..Default::default()
        }
    }

    /// Configuración para screening masivo rápido
    pub fn fast_screening() -> Self {
        Self {
            population_size: 50,
            max_generations: 20,
            mutation_rate: 0.2,
            ..Default::default()
        }
    }
}

/// Individuo en la población genética
#[derive(Debug, Clone)]
pub struct Individual {
    /// Estrategia (genotipo)
    pub strategy: StrategyAST,
    
    /// Fitness score calculado
    pub fitness: f64,
    
    /// ✨ Diversity score (semantic distance)
    pub diversity_score: f64,
    
    /// Combined fitness (performance + diversity)
    pub combined_fitness: f64,
    
    /// Generation when created
    pub generation: usize,
}

/// Resultados de evolución genética
#[derive(Debug, Clone)]
pub struct EvolutionResults {
    /// Mejor individuo encontrado
    pub best_individual: Individual,
    
    /// Población final
    pub final_population: Vec<Individual>,
    
    /// Estadísticas por generación
    pub generation_stats: Vec<GenerationStats>,
    
    /// Número total de generaciones ejecutadas
    pub generations_completed: usize,
}

/// Estadísticas de una generación
#[derive(Debug, Clone)]
pub struct GenerationStats {
    pub generation: usize,
    pub best_fitness: f64,
    pub average_fitness: f64,
    pub diversity_score: f64,
    pub population_size: usize,
}

impl GeneticGenerator {
    /// Crea un nuevo generador genético
    pub fn new(
        seed: Option<u64>,
        strategy_constraints: StrategyConstraints,
        semantic_constraints: SemanticConstraints,
        config: GeneticConfig,
    ) -> Self {
        let random_generator = RandomGenerator::new(
            seed,
            strategy_constraints.clone(),
            semantic_constraints.clone(),
        );

        Self {
            random_generator,
            strategy_constraints,
            semantic_constraints,
            config,
            population: Vec::new(),
        }
    }

    /// Constructor con configuración por defecto
    pub fn default_with_seed(seed: Option<u64>) -> Self {
        Self::new(
            seed,
            StrategyConstraints::default(),
            SemanticConstraints::default(),
            GeneticConfig::default(),
        )
    }

    /// ✨ FUTURE: Evoluciona estrategias usando algoritmo genético
    /// 
    /// Implementación completa pendiente - por ahora usa generación aleatoria mejorada
    pub fn evolve(
        &mut self,
        name_prefix: &str,
        primary_timeframe: TimeFrame,
    ) -> EvolutionResults {
        log::info!("🧬 Starting genetic evolution: {} generations, {} population", 
                  self.config.max_generations, self.config.population_size);

        // PHASE 1: Generar población inicial
        self.initialize_population(name_prefix, primary_timeframe);

        let mut generation_stats = Vec::new();
        let mut best_individual = self.population[0].clone();

        // PHASE 2: Evolución por generaciones
        for generation in 0..self.config.max_generations {
            // Calculate fitness for all individuals
            self.calculate_fitness();
            
            // Track statistics
            let stats = self.calculate_generation_stats(generation);
            generation_stats.push(stats);
            
            // Update best individual
            if let Some(current_best) = self.population.iter().max_by(|a, b| a.combined_fitness.partial_cmp(&b.combined_fitness).unwrap()) {
                if current_best.combined_fitness > best_individual.combined_fitness {
                    best_individual = current_best.clone();
                }
            }

            // Early termination if converged
            if self.has_converged() {
                log::info!("🎯 Evolution converged at generation {}", generation);
                break;
            }

            // PHASE 3: Selection, Crossover, Mutation
            self.evolve_generation(generation);
        }

        EvolutionResults {
            best_individual,
            final_population: self.population.clone(),
            generation_stats,
            generations_completed: self.config.max_generations,
        }
    }

    /// ✨ FUTURE: Genera múltiples runs del algoritmo genético
    pub fn multi_run_evolution(
        &mut self,
        runs: usize,
        name_prefix: &str,
        primary_timeframe: TimeFrame,
    ) -> Vec<EvolutionResults> {
        (0..runs)
            .map(|run| {
                log::info!("🧬 Genetic run {} of {}", run + 1, runs);
                let run_prefix = format!("{}_R{}", name_prefix, run + 1);
                self.evolve(&run_prefix, primary_timeframe)
            })
            .collect()
    }

    /// Inicializa la población con estrategias aleatorias
    fn initialize_population(&mut self, name_prefix: &str, primary_timeframe: TimeFrame) {
        self.population.clear();
        
        for i in 0..self.config.population_size {
            let name = format!("{}_G0_I{:03}", name_prefix, i + 1);
            let strategy = self.random_generator.generate_multi_timeframe(name, primary_timeframe);
            
            let individual = Individual {
                strategy,
                fitness: 0.0,
                diversity_score: 0.0,
                combined_fitness: 0.0,
                generation: 0,
            };
            
            self.population.push(individual);
        }
    }

    /// ✨ FUTURE: Calcula fitness para toda la población
    /// 
    /// Por ahora usa fitness dummy - implementación real requiere backtest engine
    fn calculate_fitness(&mut self) {
        for individual in &mut self.population {
            // PHASE 1: Calculate performance fitness (requires backtest engine)
            individual.fitness = self.calculate_performance_fitness(&individual.strategy);
            
            // PHASE 2: Calculate diversity fitness (semantic constraints)
            individual.diversity_score = self.calculate_diversity_fitness(&individual.strategy);
            
            // PHASE 3: Combine fitness scores
            individual.combined_fitness = 
                (1.0 - self.config.diversity_factor) * individual.fitness +
                self.config.diversity_factor * individual.diversity_score;
        }
    }

    /// ✨ FUTURE: Calcula fitness de performance (requiere backtest engine)
    fn calculate_performance_fitness(&self, _strategy: &StrategyAST) -> f64 {
        // Placeholder - implementación real en fases futuras con backtest engine
        // Usará métricas como: sharpe ratio, return, drawdown, win rate, etc.
        
        // Por ahora, fitness basado en características de la estrategia
        let complexity_score = 1.0 - (_strategy.complexity() as f64 / self.strategy_constraints.max_conditions as f64);
        let timeframe_score = if _strategy.is_multi_timeframe() { 1.2 } else { 1.0 };
        
        complexity_score * timeframe_score
    }

    /// ✨ FUTURE: Calcula fitness de diversidad (semantic constraints)
    fn calculate_diversity_fitness(&self, strategy: &StrategyAST) -> f64 {
        // Placeholder - implementación real en Phase 3 con correlation matrix
        
        // Por ahora, diversidad basada en características básicas
        let timeframe_diversity = strategy.timeframe_count() as f64 / 3.0; // Max 3 timeframes
        let indicator_diversity = strategy.all_indicators().len() as f64 / self.strategy_constraints.max_indicators as f64;
        
        (timeframe_diversity + indicator_diversity) / 2.0
    }

    /// Calcula estadísticas de la generación actual
    fn calculate_generation_stats(&self, generation: usize) -> GenerationStats {
        let fitnesses: Vec<f64> = self.population.iter().map(|i| i.combined_fitness).collect();
        let best_fitness = fitnesses.iter().fold(0.0f64, |a, &b| a.max(b));
        let average_fitness = fitnesses.iter().sum::<f64>() / fitnesses.len() as f64;
        
        let diversity_scores: Vec<f64> = self.population.iter().map(|i| i.diversity_score).collect();
        let diversity_score = diversity_scores.iter().sum::<f64>() / diversity_scores.len() as f64;

        GenerationStats {
            generation,
            best_fitness,
            average_fitness,
            diversity_score,
            population_size: self.population.len(),
        }
    }

    /// Verifica si el algoritmo ha convergido
    fn has_converged(&self) -> bool {
        // Convergencia simple - implementación más sofisticada en fases futuras
        let fitnesses: Vec<f64> = self.population.iter().map(|i| i.combined_fitness).collect();
        let min_fitness = fitnesses.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_fitness = fitnesses.iter().fold(0.0f64, |a, &b| a.max(b));
        
        max_fitness - min_fitness < 0.001 // Muy baja varianza = convergencia
    }

    /// ✨ FUTURE: Evoluciona una generación (selection, crossover, mutation)
    fn evolve_generation(&mut self, generation: usize) {
        // Por ahora, implementación simplificada - reemplaza con nuevos aleatorios
        // Implementación real de crossover y mutation en fases futuras
        
        // Mantener elite
        self.population.sort_by(|a, b| b.combined_fitness.partial_cmp(&a.combined_fitness).unwrap());
        let elite_count = (self.population.len() as f64 * self.config.elite_percentage) as usize;
        
        // Reemplazar el resto con nuevos individuos aleatorios (temporal)
        for i in elite_count..self.population.len() {
            let name = format!("Evolved_G{}_I{:03}", generation + 1, i + 1);
            let strategy = self.random_generator.generate_multi_timeframe(
                name, 
                self.population[i].strategy.primary_timeframe
            );
            
            self.population[i] = Individual {
                strategy,
                fitness: 0.0,
                diversity_score: 0.0,
                combined_fitness: 0.0,
                generation: generation + 1,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genetic_config() {
        let config = GeneticConfig::default();
        assert_eq!(config.population_size, 100);
        assert!(config.diversity_factor > 0.0);
        
        let diversity_config = GeneticConfig::diversity_focused();
        assert!(diversity_config.diversity_factor > config.diversity_factor);
        
        let performance_config = GeneticConfig::performance_focused();
        assert!(performance_config.diversity_factor < config.diversity_factor);
    }

    #[test]
    fn test_genetic_generator_creation() {
        let generator = GeneticGenerator::default_with_seed(Some(42));
        assert_eq!(generator.config.population_size, 100);
        assert_eq!(generator.population.len(), 0); // Empty until evolution starts
    }

    #[test]
    fn test_population_initialization() {
        let mut generator = GeneticGenerator::default_with_seed(Some(42));
        generator.initialize_population("Test", TimeFrame::M5);
        
        assert_eq!(generator.population.len(), 100);
        
        for individual in &generator.population {
            assert_eq!(individual.generation, 0);
            assert_eq!(individual.strategy.primary_timeframe, TimeFrame::M5);
            assert!(individual.strategy.name.starts_with("Test_G0_I"));
        }
    }

    #[test]
    fn test_fitness_calculation() {
        let mut generator = GeneticGenerator::default_with_seed(Some(42));
        generator.initialize_population("Fitness", TimeFrame::H1);
        
        generator.calculate_fitness();
        
        for individual in &generator.population {
            assert!(individual.fitness >= 0.0);
            assert!(individual.diversity_score >= 0.0);
            assert!(individual.combined_fitness >= 0.0);
        }
    }

    #[test]
    fn test_generation_stats() {
        let mut generator = GeneticGenerator::default_with_seed(Some(42));
        generator.initialize_population("Stats", TimeFrame::M15);
        generator.calculate_fitness();
        
        let stats = generator.calculate_generation_stats(0);
        
        assert_eq!(stats.generation, 0);
        assert_eq!(stats.population_size, 100);
        assert!(stats.best_fitness >= stats.average_fitness);
        assert!(stats.average_fitness >= 0.0);
    }

    #[test]
    fn test_evolution_basic() {
        let mut generator = GeneticGenerator::new(
            Some(42),
            StrategyConstraints::default(),
            SemanticConstraints::default(),
            GeneticConfig::fast_screening(), // Smaller/faster for testing
        );
        
        let results = generator.evolve("Evolution", TimeFrame::M5);
        
        assert_eq!(results.final_population.len(), 50); // fast_screening population
        assert!(results.generations_completed > 0);
        assert!(!results.generation_stats.is_empty());
        assert!(results.best_individual.combined_fitness >= 0.0);
    }

    #[test]
    fn test_multi_run_evolution() {
        let mut generator = GeneticGenerator::new(
            Some(42),
            StrategyConstraints::default(),
            SemanticConstraints::default(),
            GeneticConfig::fast_screening(),
        );
        
        let results = generator.multi_run_evolution(2, "MultiRun", TimeFrame::H1);
        
        assert_eq!(results.len(), 2);
        for result in results {
            assert!(result.generations_completed > 0);
            assert!(!result.final_population.is_empty());
        }
    }

    #[test]
    fn test_convergence_detection() {
        let mut generator = GeneticGenerator::default_with_seed(Some(42));
        generator.initialize_population("Convergence", TimeFrame::D1);
        
        // Set all fitness to same value to trigger convergence
        for individual in &mut generator.population {
            individual.combined_fitness = 0.5;
        }
        
        assert!(generator.has_converged());
    }

    #[test]
    fn test_individual_creation() {
        let strategy = StrategyAST::new("Test Individual".to_string(), TimeFrame::M5);
        
        let individual = Individual {
            strategy: strategy.clone(),
            fitness: 0.8,
            diversity_score: 0.6,
            combined_fitness: 0.7,
            generation: 5,
        };
        
        assert_eq!(individual.strategy.name, "Test Individual");
        assert_eq!(individual.fitness, 0.8);
        assert_eq!(individual.generation, 5);
    }
}
