use darwinx_indicators::registry;
use darwinx_indicators::metadata::IndicatorCategory;

fn main() {
    println!("🎯 DarwinX Indicators Registry\n");
    println!("{}", "=".repeat(60));
    
    // Estadísticas
    let stats = registry::stats();
    println!("\n📊 ESTADÍSTICAS:");
    println!("   Total: {} indicadores", stats.total);
    println!("   ├─ Trend: {}", stats.trend);
    println!("   ├─ Momentum: {}", stats.momentum);
    println!("   ├─ Volatility: {}", stats.volatility);
    println!("   └─ Volume: {}", stats.volume);
    
    // Listar todos
    println!("\n📋 INDICADORES DISPONIBLES:");
    let mut names = registry::all_names();
    names.sort();
    for name in &names {
        let meta = registry::get(name).unwrap();
        println!("   • {:<20} - {} ({} params)", 
            meta.name, 
            meta.description, 
            meta.parameters.len()
        );
    }
    
    // Por categoría
    println!("\n🔵 TREND:");
    for meta in registry::by_category(IndicatorCategory::Trend) {
        println!("   • {}", meta.name);
    }
    
    println!("\n🟢 MOMENTUM:");
    for meta in registry::by_category(IndicatorCategory::Momentum) {
        println!("   • {}", meta.name);
    }
    
    println!("\n🟡 VOLATILITY:");
    for meta in registry::by_category(IndicatorCategory::Volatility) {
        println!("   • {}", meta.name);
    }
    
    println!("\n🟠 VOLUME:");
    for meta in registry::by_category(IndicatorCategory::Volume) {
        println!("   • {}", meta.name);
    }
    
    println!("\n{}", "=" .repeat(60));
    println!("✅ Registry funcionando correctamente");
}