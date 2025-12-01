//! Ejemplos prácticos de uso del Strategy Generator Multi-Timeframe
//!
//! Este módulo contiene ejemplos completos que demuestran las nuevas capacidades
//! multi-timeframe del generador de estrategias.

use darwinx_strategy_generator::*;
use darwinx_core::TimeFrame;

/// Ejemplo 1: Estrategia Golden Cross Multi-Timeframe Manual
pub fn example_golden_cross_multi_tf() {
    println!("🎯 Ejemplo 1: Golden Cross Multi-Timeframe");
    println!("==========================================");

    let strategy = StrategyBuilder::new("Golden Cross Multi-TF".to_string(), TimeFrame::M5)
        // Señal principal en timeframe actual (5m)
        .add_entry_condition_with_timeframe(
            ConditionBuilder::crosses_above("ema", vec![50.0], "ema", vec![200.0]),
            TimeframeCategory::Current
        )
        // Filtro de momentum en timeframe actual (5m)
        .add_entry_condition_with_timeframe(
            ConditionBuilder::above("rsi", vec![14.0], 50.0),
            TimeframeCategory::Current
        )
        // Confirmación de tendencia en timeframe superior (15m)
        .add_entry_condition_with_timeframe(
            ConditionBuilder::above_price("sma", vec![200.0]),
            TimeframeCategory::Medium
        )
        // Salida por death cross
        .add_exit_condition_with_timeframe(
            ConditionBuilder::crosses_below("ema", vec![50.0], "ema", vec![200.0]),
            TimeframeCategory::Current
        )
        // Salida por RSI sobrecomprado
        .add_exit_condition_with_timeframe(
            ConditionBuilder::above("rsi", vec![14.0], 70.0),
            TimeframeCategory::Current
        )
        .entry_operator(LogicalOperator::And)
        .exit_operator(LogicalOperator::Or)
        .build();

    println!("{}", strategy.display_summary());
    println!("Timeframes used: {:?}", strategy.used_timeframe_categories());
    println!("Absolute timeframes: {:?}", strategy.timeframe_mapping());
    println!();
}

/// Ejemplo 2: Generación Automática con Constraints
pub fn example_automatic_generation() {
    println!("🎯 Ejemplo 2: Generación Automática Multi-Timeframe");
    println!("================================================");

    // Configurar constraints para estrategias complejas
    let strategy_constraints = StrategyConstraints::relaxed(); // Hasta 3 timeframes
    let semantic_constraints = SemanticConstraints::moderate(); // 70% max correlation

    // Configurar distribución para favorizar multi-timeframe
    let timeframe_dist = TimeframeDistribution::multi_timeframe_focused();

    let mut generator = RandomGenerator::new(
        Some(42), // Seed para reproducibilidad
        strategy_constraints,
        semantic_constraints,
    ).with_timeframe_distribution(timeframe_dist);

    // Generar batch de estrategias
    let strategies = generator.generate_batch(10, "AutoGen", TimeFrame::H1);

    println!("Generated {} strategies:", strategies.len());
    for (i, strategy) in strategies.iter().enumerate() {
        let tf_info = if strategy.is_multi_timeframe() {
            format!("Multi-TF ({} timeframes)", strategy.timeframe_count())
        } else {
            "Single-TF".to_string()
        };
        
        println!("  {}: {} - {} - {} conditions", 
                 i + 1, strategy.name, tf_info, strategy.complexity());
    }

    // Estadísticas
    let multi_tf_count = strategies.iter().filter(|s| s.is_multi_timeframe()).count();
    println!("\nStats: {}/{} multi-timeframe ({:.1}%)", 
             multi_tf_count, strategies.len(), 
             (multi_tf_count as f64 / strategies.len() as f64) * 100.0);
    println!();
}

/// Ejemplo 3: Validación Detallada Multi-Timeframe
pub fn example_detailed_validation() {
    println!("🎯 Ejemplo 3: Validación Detallada Multi-Timeframe");
    println!("================================================");

    // Crear estrategia compleja multi-timeframe
    let strategy = StrategyBuilder::new("Complex Multi-TF".to_string(), TimeFrame::M15)
        .add_entry_condition_with_timeframe(
            ConditionBuilder::indicator_above_multi_tf(
                "ema", vec![12.0], TimeframeCategory::Current,   // EMA 12 en 15m
                "sma", vec![26.0], TimeframeCategory::Medium     // SMA 26 en 1h
            ),
            TimeframeCategory::Current
        )
        .add_entry_condition_with_timeframe(
            ConditionBuilder::above("rsi", vec![14.0], 40.0),
            TimeframeCategory::Current
        )
        .add_entry_condition_with_timeframe(
            ConditionBuilder::above("volume", vec![], 1.5),
            TimeframeCategory::High  // Volumen en 4h para contexto
        )
        .add_exit_condition_with_timeframe(
            ConditionBuilder::below("rsi", vec![14.0], 30.0),
            TimeframeCategory::Current
        )
        .entry_operator(LogicalOperator::And)
        .build();

    // Validar con reporte detallado
    let validator = StrategyValidator::new(StrategyConstraints::default());
    let report = validator.validate_detailed(&strategy);

    if report.is_valid() {
        println!("✅ Strategy validation: PASSED");
    } else {
        println!("❌ Strategy validation: FAILED");
        for error in &report.errors {
            println!("  Error: {}", error);
        }
    }

    println!("\nValidation Info:");
    for info in &report.info {
        println!("  ℹ️  {}", info);
    }

    if !report.warnings.is_empty() {
        println!("\nWarnings:");
        for warning in &report.warnings {
            println!("  ⚠️  {}", warning);
        }
    }
    println!();
}

/// Ejemplo 4: Diferentes Distribuciones de Timeframes
pub fn example_timeframe_distributions() {
    println!("🎯 Ejemplo 4: Diferentes Distribuciones de Timeframes");
    println!("==================================================");

    let distributions = vec![
        ("Single-TF Focused", TimeframeDistribution::single_timeframe_focused()),
        ("Multi-TF Focused", TimeframeDistribution::multi_timeframe_focused()),
        ("Balanced", TimeframeDistribution::balanced()),
        ("Conservative", TimeframeDistribution::conservative()),
    ];

    for (name, dist) in distributions {
        println!("\n{} Distribution:", name);
        println!("  Current probability: {:.1}%", dist.current_probability * 100.0);
        println!("  Medium probability: {:.1}%", dist.medium_probability * 100.0);
        println!("  High probability: {:.1}%", dist.high_probability * 100.0);
        println!("  Multi-TF probability: {:.1}%", dist.multi_timeframe_probability * 100.0);

        // Generar pequeño sample
        let mut generator = RandomGenerator::default_with_seed(Some(42))
            .with_timeframe_distribution(dist);
        
        let strategies = generator.generate_batch(20, "Sample", TimeFrame::M5);
        let multi_count = strategies.iter().filter(|s| s.is_multi_timeframe()).count();
        
        println!("  Sample result: {}/20 multi-TF ({:.1}%)", 
                 multi_count, (multi_count as f64 / 20.0) * 100.0);
    }
    println!();
}

/// Ejemplo 5: Estrategias Pre-construidas Multi-Timeframe
pub fn example_prebuilt_strategies() {
    println!("🎯 Ejemplo 5: Estrategias Pre-construidas Multi-Timeframe");
    println!("========================================================");

    // Golden Cross Multi-TF
    let golden_cross = StrategyBuilder::golden_cross_multi_tf(
        "Golden Cross Professional".to_string(),
        TimeFrame::H1,
        50,  // EMA short period
        200, // EMA long period
        14   // RSI period
    ).build();

    println!("Golden Cross Strategy:");
    println!("{}", golden_cross.display_summary());
    println!("Entry conditions: {}", golden_cross.entry_rules.conditions.len());
    println!("Exit conditions: {}", golden_cross.exit_rules.conditions.len());

    // Mean Reversion Multi-TF
    let mean_reversion = StrategyBuilder::mean_reversion_multi_tf(
        "Mean Reversion Professional".to_string(),
        TimeFrame::M15,
        14, // RSI period
        50, // SMA period
        20  // Volume SMA period
    ).build();

    println!("\nMean Reversion Strategy:");
    println!("{}", mean_reversion.display_summary());
    println!("Entry conditions: {}", mean_reversion.entry_rules.conditions.len());
    println!("Exit conditions: {}", mean_reversion.exit_rules.conditions.len());
    println!();
}

/// Ejemplo 6: Cross-Timeframe Batch Generation
pub fn example_cross_timeframe_generation() {
    println!("🎯 Ejemplo 6: Cross-Timeframe Batch Generation");
    println!("===========================================");

    let mut generator = RandomGenerator::default_with_seed(Some(123));
    
    // Generar estrategias para diferentes timeframes principales
    let timeframes = vec![TimeFrame::M5, TimeFrame::M15, TimeFrame::H1, TimeFrame::H4];
    let strategies = generator.generate_cross_timeframe_batch(3, "CrossTF", &timeframes);

    println!("Generated {} strategies across {} timeframes:", 
             strategies.len(), timeframes.len());

    let mut by_timeframe: std::collections::HashMap<TimeFrame, Vec<&StrategyAST>> = 
        std::collections::HashMap::new();
    
    for strategy in &strategies {
        by_timeframe.entry(strategy.primary_timeframe)
            .or_insert_with(Vec::new)
            .push(strategy);
    }

    for (tf, strategies) in by_timeframe {
        let multi_tf_count = strategies.iter().filter(|s| s.is_multi_timeframe()).count();
        println!("  {:?}: {} strategies ({} multi-TF)", 
                 tf, strategies.len(), multi_tf_count);
        
        for strategy in strategies {
            println!("    - {}: {} timeframes, {} conditions", 
                     strategy.name, 
                     strategy.timeframe_count(),
                     strategy.complexity());
        }
    }
    println!();
}

/// Ejemplo 7: Genetic Algorithm Base (Phase 4 Preview)
pub fn example_genetic_algorithm() {
    println!("🎯 Ejemplo 7: Genetic Algorithm (Base Implementation)");
    println!("===================================================");

    let genetic_config = GeneticConfig {
        population_size: 20,     // Small for example
        max_generations: 5,      // Quick run
        mutation_rate: 0.1,
        crossover_rate: 0.7,
        elite_percentage: 0.2,
        diversity_factor: 0.4,   // Balance performance + diversity
    };

    let mut genetic_generator = GeneticGenerator::new(
        Some(42),
        StrategyConstraints::moderate(),
        SemanticConstraints::moderate(),
        genetic_config,
    );

    println!("Running genetic evolution...");
    let results = genetic_generator.evolve("Genetic", TimeFrame::M5);

    println!("Evolution completed!");
    println!("  Generations: {}", results.generations_completed);
    println!("  Final population: {}", results.final_population.len());
    println!("  Best fitness: {:.3}", results.best_individual.combined_fitness);
    println!("  Best strategy: {}", results.best_individual.strategy.name);
    println!("  Best complexity: {} conditions", results.best_individual.strategy.complexity());
    
    if results.best_individual.strategy.is_multi_timeframe() {
        println!("  Best is multi-TF: {} timeframes", 
                 results.best_individual.strategy.timeframe_count());
    }

    // Show evolution progress
    println!("\nEvolution Progress:");
    for (i, stats) in results.generation_stats.iter().enumerate() {
        println!("  Gen {}: Best={:.3}, Avg={:.3}, Diversity={:.3}", 
                 i, stats.best_fitness, stats.average_fitness, stats.diversity_score);
    }
    println!();
}

/// Función principal que ejecuta todos los ejemplos
pub fn run_all_examples() {
    println!("🧬 DarwinX Strategy Generator v2.1 - Multi-Timeframe Examples");
    println!("============================================================");
    println!();

    example_golden_cross_multi_tf();
    example_automatic_generation();
    example_detailed_validation();
    example_timeframe_distributions();
    example_prebuilt_strategies();
    example_cross_timeframe_generation();
    example_genetic_algorithm();

    println!("✅ All examples completed successfully!");
    println!("📚 Check the source code for implementation details.");
}

#[cfg(test)]
mod example_tests {
    use super::*;

    #[test]
    fn test_all_examples_run_without_panic() {
        // Test que todos los ejemplos se ejecutan sin panic
        example_golden_cross_multi_tf();
        example_automatic_generation();
        example_detailed_validation();
        example_timeframe_distributions();
        example_prebuilt_strategies();
        example_cross_timeframe_generation();
        example_genetic_algorithm();
    }

    #[test]
    fn test_golden_cross_example_properties() {
        let strategy = StrategyBuilder::golden_cross_multi_tf(
            "Test Golden Cross".to_string(),
            TimeFrame::H1,
            50, 200, 14
        ).build();

        assert!(strategy.is_multi_timeframe());
        assert!(strategy.entry_rules.conditions.len() >= 3);
        assert!(strategy.exit_rules.conditions.len() >= 2);
    }

    #[test]
    fn test_automatic_generation_example() {
        let mut generator = RandomGenerator::default_with_seed(Some(42));
        let strategies = generator.generate_batch(5, "Test", TimeFrame::M5);
        
        assert_eq!(strategies.len(), 5);
        for strategy in strategies {
            assert_eq!(strategy.primary_timeframe, TimeFrame::M5);
            assert!(!strategy.entry_rules.conditions.is_empty());
        }
    }
}
