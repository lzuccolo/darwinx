//! Ejemplo simple de cómo ejecutar el generador de estrategias
//!
//! Ejecutar con: cargo run --package darwinx-generator --example run_generator

use darwinx_generator::*;

fn main() {
    println!("🧬 DarwinX Strategy Generator");
    println!("=============================\n");

    // 1. Crear generador
    let generator = RandomGenerator::new();

    // 2. Generar una estrategia individual
    println!("📝 Generando estrategia individual...");
    let strategy = generator.generate("Mi Primera Estrategia".to_string());
    println!("   Nombre: {}", strategy.name);
    println!("   Timeframe: {:?}", strategy.timeframe);
    println!("   Condiciones entrada: {}", strategy.entry_rules.conditions.len());
    println!("   Condiciones salida: {}", strategy.exit_rules.conditions.len());
    println!();

    // 3. Generar múltiples estrategias en batch
    println!("📦 Generando batch de 10 estrategias...\n");
    let strategies = generator.generate_batch(10);

    // 4. Mostrar todas las estrategias
    for (i, strategy) in strategies.iter().enumerate() {
        println!("{}. {}", i + 1, strategy.name);
        println!("   Timeframe: {:?}", strategy.timeframe);
        println!("   Entrada: {} condiciones", strategy.entry_rules.conditions.len());
        println!("   Salida: {} condiciones", strategy.exit_rules.conditions.len());
        println!();
    }

    // 5. Estadísticas
    let total_conditions: usize = strategies.iter()
        .map(|s| s.entry_rules.conditions.len() + s.exit_rules.conditions.len())
        .sum();
    
    let avg_conditions = total_conditions as f64 / strategies.len() as f64;

    println!("📊 Estadísticas:");
    println!("   Total estrategias: {}", strategies.len());
    println!("   Promedio condiciones por estrategia: {:.2}", avg_conditions);
    println!("   Total condiciones: {}", total_conditions);
}

