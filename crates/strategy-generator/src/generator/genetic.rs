//! Algoritmo genético - 100% DINÁMICO usando registry
//!
//! Este módulo implementa un algoritmo genético completo para evolucionar
//! estrategias de trading, incluyendo selección por torneo, crossover,
//! mutación y elitismo.

use crate::ast::nodes::*;
use crate::generator::random::RandomGenerator;
use darwinx_indicators::registry;
use rand::prelude::*;

// Configuración del algoritmo genético
//
// Controla el comportamiento del proceso evolutivo, incluyendo el tamaño de la
// población, número de generaciones, y tasas de mutación y elitismo.
//
// # Ejemplos
//
// ```
// use darwinx_generator::GeneticConfig;
//
// // Configuración por defecto
// let config = GeneticConfig::default();
//
// // Configuración personalizada para evolución rápida
// let quick_config = GeneticConfig {
//     population_size: 30,
//     generations: 20,
//     mutation_rate: 0.2,
//     elite_size: 3,
//     tournament_size: 3,
// };
// ```
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

// Generador genético de estrategias de trading
//
// Implementa un algoritmo genético completo para evolucionar estrategias de trading
// usando selección por torneo, crossover, mutación y elitismo.
//
// # Características
//
// - **100% dinámico**: Usa el registry de indicadores sin hardcoding
// - **5 tipos de mutación**: Condiciones, parámetros, operadores, comparadores
// - **Elitismo configurable**: Preserva las mejores estrategias
// - **Convergencia anticipada**: Detecta estancamiento y termina antes
//
// # Ejemplos
//
// ```no_run
// use darwinx_generator::{GeneticGenerator, GeneticConfig};
//
// let config = GeneticConfig::default();
// let generator = GeneticGenerator::new(config);
//
// let population = generator.generate_population(100);
//
// let fitness_fn = |strategy| {
//     strategy.complexity() as f64
// };
//
// let best = generator.evolve(population, fitness_fn);
// println!("Mejor estrategia: {}", best[0].name);
// ```
pub struct GeneticGenerator {
    config: GeneticConfig,
    random_gen: RandomGenerator,
}

impl GeneticGenerator {
    // Crea un nuevo generador genético con la configuración especificada
    //
    // # Argumentos
    //
    // * `config` - Configuración del algoritmo genético
    pub fn new(config: GeneticConfig) -> Self {
        Self {
            config,
            random_gen: RandomGenerator::new(),
        }
    }

    // Genera una población inicial de estrategias aleatorias
    //
    // Utiliza el generador aleatorio interno para crear estrategias válidas
    // que respetan los constraints configurados.
    //
    // # Argumentos
    //
    // * `count` - Número de estrategias a generar
    //
    // # Returns
    //
    // Vector con `count` estrategias válidas generadas aleatoriamente
    pub fn generate_population(&self, count: usize) -> Vec<StrategyAST> {
        self.random_gen.generate_batch(count)
    }

    // Cruza dos estrategias padre para crear un hijo (crossover)
    //
    // Implementa crossover de un punto para las condiciones de entrada,
    // y selección aleatoria para las condiciones de salida y operadores lógicos.
    //
    // # Argumentos
    //
    // * `parent1` - Primera estrategia padre
    // * `parent2` - Segunda estrategia padre
    //
    // # Returns
    //
    // Nueva estrategia (hijo) que combina características de ambos padres
    pub fn crossover(&self, parent1: &StrategyAST, parent2: &StrategyAST) -> StrategyAST {
        let mut rng = rand::thread_rng();
        
        // Generar nombre corto usando hash de los padres para evitar nombres exponencialmente largos
        let name_hash = format!("{:x}", 
            (parent1.name.len() as u64).wrapping_mul(31) 
            ^ (parent2.name.len() as u64).wrapping_mul(17)
        );
        let short_name = format!("Evolved_{}", &name_hash[..8.min(name_hash.len())]);
        
        let mut child = StrategyAST::new(
            short_name,
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

    // Muta una estrategia de forma aleatoria
    //
    // Implementa 5 tipos diferentes de mutación para mantener diversidad genética:
    //
    // 1. **Reemplazar condición de entrada** (probabilidad: mutation_rate)
    // 2. **Reemplazar condición de salida** (probabilidad: mutation_rate)
    // 3. **Cambiar operador lógico** (probabilidad: mutation_rate * 0.5)
    // 4. **Ajustar parámetros de indicadores** (probabilidad: mutation_rate * 0.3)
    // 5. **Cambiar comparador** (probabilidad: mutation_rate * 0.2)
    //
    // # Argumentos
    //
    // * `strategy` - Estrategia a mutar (modificada in-place)
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

    // Mutación de parámetros de indicadores existentes
    // 
    // Ajusta los parámetros dentro de rangos válidos según metadata
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

    // Muta los parámetros de un indicador usando metadata del registry
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

    // Mutación de comparador
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

    // Selecciona una estrategia mediante torneo (tournament selection)
    //
    // Elige aleatoriamente `tournament_size` candidatos de la población y
    // retorna el que tiene mejor fitness.
    //
    // # Argumentos
    //
    // * `population` - Población de estrategias
    // * `fitness_fn` - Función que calcula el fitness de cada estrategia
    //
    // # Returns
    //
    // Estrategia ganadora del torneo (clonada)
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

    // Evoluciona una población de estrategias usando algoritmo genético
    //
    // Implementa un loop evolutivo completo que incluye evaluación de fitness,
    // selección, crossover, mutación y elitismo.
    //
    // # Argumentos
    //
    // * `initial_population` - Población inicial de estrategias
    // * `fitness_fn` - Función que evalúa el fitness (mayor es mejor)
    //
    // # Returns
    //
    // Vector de estrategias ordenado por fitness (mejor primero)
    //
    // # Proceso Evolutivo
    //
    // Para cada generación:
    // 1. Evaluar fitness de toda la población
    // 2. Preservar elite (mejores estrategias)
    // 3. Generar nueva población mediante selección, crossover y mutación
    // 4. Reemplazar población anterior
    // 5. Verificar convergencia
    //
    // # Convergencia
    //
    // El algoritmo termina antes si no hay mejora por 25 generaciones consecutivas.
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

            // Ordenar por fitness (descendente)
            fitness_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

            // Actualizar mejor fitness
            let current_best = fitness_scores[0].1;
            if current_best > best_fitness {
                best_fitness = current_best;
                generations_without_improvement = 0;
            } else {
                generations_without_improvement += 1;
            }

            // Logging cada 10 generaciones
            if generation % 10 == 0 {
                let avg_fitness = fitness_scores.iter().map(|(_, f)| f).sum::<f64>() / fitness_scores.len() as f64;
                println!(
                    "Generación {}/{}, Best: {:.4}, Avg: {:.4}",
                    generation + 1,
                    self.config.generations,
                    current_best,
                    avg_fitness
                );
            }

            // Convergencia anticipada (sin mejora por 25 generaciones)
            if generations_without_improvement >= 25 {
                println!("Convergencia alcanzada en generación {}", generation + 1);
                break;
            }

            // Crear nueva generación
            let mut new_population = Vec::new();

            // Elitismo: preservar las mejores estrategias
            for i in 0..self.config.elite_size.min(population.len()) {
                let idx = fitness_scores[i].0;
                new_population.push(population[idx].clone());
            }

            // Generar resto de la población
            let mut child_counter = 0;
            while new_population.len() < self.config.population_size {
                // Selección de padres
                let parent1 = self.tournament_selection(&population, &fitness_fn);
                let parent2 = self.tournament_selection(&population, &fitness_fn);

                // Crossover
                let mut child = self.crossover(&parent1, &parent2);
                
                // Actualizar nombre con generación y contador para mejor trazabilidad
                child.name = format!("Evolved_G{}_C{:04}", generation + 1, child_counter);
                child_counter += 1;

                // Mutación
                self.mutate(&mut child);

                new_population.push(child);
            }

            // Reemplazar población
            population = new_population;
        }

        // Ordenar población final por fitness
        let mut fitness_scores: Vec<(usize, f64)> = population
            .iter()
            .enumerate()
            .map(|(idx, strategy)| (idx, fitness_fn(strategy)))
            .collect();

        fitness_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Retornar población ordenada
        fitness_scores
            .iter()
            .map(|(idx, _)| population[*idx].clone())
            .collect()
    }

    // Funciones auxiliares privadas

    fn random_condition(&self, rng: &mut impl Rng) -> Condition {
        Condition {
            indicator: self.random_indicator(rng),
            comparison: self.random_comparison(rng),
            value: self.random_value(rng),
        }
    }

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
        let _original = pop[0].clone();

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
        let final_scores: Vec<_> = evolved_pop.iter()
            .map(|s| fitness_fn(s))
            .collect();

        // El mejor final debe ser al menos tan bueno como el mejor inicial
        assert!(final_scores[0] >= initial_scores[0].1);
    }
}