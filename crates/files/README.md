# DarwinX Strategy Generator v2.1 - Multi-Timeframe Edition

## 🎯 Resumen de Actualización

Esta actualización implementa la **Fase 1 - Semana 2** del roadmap v2.1, agregando soporte completo para estrategias multi-timeframe al crate `strategy-generator`.

## ✨ Nuevas Características v2.1

### 🧬 Multi-Timeframe Support
- **TimeframeCategory**: Sistema de timeframes relativos (Current/Medium/High)
- **Enhanced StrategyAST**: Soporte completo para múltiples timeframes
- **Smart Builder**: Métodos fluidos para crear estrategias multi-TF
- **Intelligent Validation**: Validación específica para consistencia multi-TF

### 🏗️ Arquitectura Rust 2024
- **Sin mod.rs**: Estructura moderna con declaración de módulos
- **Modular Design**: Separación clara de responsabilidades
- **Type Safety**: API type-safe para timeframes y constraints

### 🧠 Semantic Constraints (Base)
- **Structure Ready**: Base para Phase 3 implementation
- **Anti-Correlation**: Framework para evitar indicadores correlacionados
- **Dynamic Limits**: Límites basados en categorías de indicadores

## 📁 Estructura del Proyecto

```
strategy-generator/
├── Cargo.toml                    # Dependencies actualizadas
├── lib.rs                       # Main module con re-exports
├── ast.rs                       # AST module declaration
├── constraints.rs               # Constraints module declaration  
├── generator.rs                 # Generator module declaration
│
├── ast/                         # AST implementation
│   ├── nodes.rs                 # ✨ Multi-TF AST nodes
│   ├── builder.rs               # ✨ Multi-TF builder methods
│   └── validator.rs             # ✨ Multi-TF validation
│
├── constraints/                 # Constraints system
│   ├── strategy.rs              # ✨ Updated basic constraints
│   └── semantic.rs              # ✨ Base semantic constraints
│
└── generator/                   # Strategy generators
    ├── random.rs                # ✨ Multi-TF random generator
    └── genetic.rs               # ✨ Base genetic algorithm
```

## 🚀 Quick Start

### Crear Strategy Multi-Timeframe

```rust
use darwinx_strategy_generator::*;
use darwinx_core::TimeFrame;

// Golden Cross con contexto multi-timeframe
let strategy = StrategyBuilder::new("Golden Cross Multi-TF".to_string(), TimeFrame::M5)
    .add_entry_condition_with_timeframe(
        ConditionBuilder::crosses_above("ema", vec![50.0], "ema", vec![200.0]),
        TimeframeCategory::Current  // EMA cross en timeframe principal (5m)
    )
    .add_entry_condition_with_timeframe(
        ConditionBuilder::above("rsi", vec![14.0], 50.0),
        TimeframeCategory::Current  // RSI filter en 5m
    )
    .add_entry_condition_with_timeframe(
        ConditionBuilder::above_price("sma", vec![200.0]),
        TimeframeCategory::Medium   // Trend confirmation en 15m
    )
    .build();

println!("{}", strategy.display_summary());
// Output: Strategy: Golden Cross Multi-TF (Primary: M5)
//         Multi-timeframe: Current@M5(2) Medium@M15(1)
//         Complexity: 3 conditions (3 entry, 0 exit)
```

### Generación Automática Multi-Timeframe

```rust
use darwinx_strategy_generator::*;

let mut generator = RandomGenerator::default_with_seed(Some(42))
    .with_timeframe_distribution(TimeframeDistribution::multi_timeframe_focused());

// Generar batch de estrategias multi-timeframe
let strategies = generator.generate_batch(100, "MultiTF", TimeFrame::H1);

let multi_tf_count = strategies.iter()
    .filter(|s| s.is_multi_timeframe())
    .count();

println!("Generated {} multi-timeframe strategies out of {}", 
         multi_tf_count, strategies.len());
```

### Validación Multi-Timeframe

```rust
let validator = StrategyValidator::new(StrategyConstraints::default());
let report = validator.validate_detailed(&strategy);

if report.is_valid() {
    println!("✅ Strategy is valid!");
    for info in report.info {
        println!("ℹ️  {}", info);
    }
} else {
    for error in report.errors {
        println!("❌ {}", error);
    }
}
```

## 📊 TimeFrame Mapping

El sistema usa categorías semánticas que se mapean automáticamente:

| Primary TF | Current | Medium | High | Use Case |
|------------|---------|--------|------|----------|
| **1m** | 1m | 5m | 1h | Scalping + Context |
| **5m** | 5m | 15m | 1h | Day trading |
| **15m** | 15m | 1h | 4h | Swing trading |
| **1h** | 1h | 4h | 1d | Position trading |
| **4h** | 4h | 1d | 1w | Long-term |
| **1d** | 1d | 1w | 1M | Investment |

## 🎛️ Configuración Avanzada

### Strategy Constraints

```rust
// Para estrategias simples single-timeframe
let constraints = StrategyConstraints::strict(); // max_timeframes: 1

// Para estrategias multi-timeframe básicas  
let constraints = StrategyConstraints::moderate(); // max_timeframes: 2

// Para estrategias complejas multi-timeframe
let constraints = StrategyConstraints::relaxed(); // max_timeframes: 3
```

### Timeframe Distribution

```rust
// Favorizar estrategias multi-timeframe
let dist = TimeframeDistribution::multi_timeframe_focused();

// Solo single-timeframe
let dist = TimeframeDistribution::single_timeframe_focused();

// Balance
let dist = TimeframeDistribution::balanced();
```

### Semantic Constraints (Base)

```rust
// Máxima diversidad (Phase 3)
let semantic = SemanticConstraints::strict(); // 50% max correlation

// Diversidad moderada
let semantic = SemanticConstraints::moderate(); // 70% max correlation

// Permite más correlación
let semantic = SemanticConstraints::relaxed(); // 85% max correlation
```

## 🧪 Testing

El crate incluye tests comprehensivos:

```bash
# Run all tests
cargo test

# Test specific features
cargo test multi_timeframe
cargo test backward_compatibility
cargo test validation
```

### Coverage Status

- ✅ **Multi-timeframe AST**: 100% tested
- ✅ **Builder methods**: 100% tested  
- ✅ **Validation**: 100% tested
- ✅ **Random generation**: 100% tested
- ✅ **Backward compatibility**: 100% tested
- 🟡 **Genetic algorithm**: Base structure tested
- 🟡 **Semantic constraints**: Structure tested (Phase 3 pending)

## 🔄 Backward Compatibility

Todas las APIs v2.0 siguen funcionando:

```rust
// ✅ Código v2.0 sigue funcionando
let strategy = StrategyBuilder::new("Legacy".to_string(), TimeFrame::H1)
    .add_entry_condition(ConditionBuilder::above("rsi", vec![14.0], 70.0))
    .add_exit_condition(ConditionBuilder::below("rsi", vec![14.0], 30.0))
    .build();

// ✅ Validation legacy sigue funcionando
let validator = StrategyValidator::new(StrategyConstraints::default());
let result = validator.validate(&strategy); // Returns Result<(), Vec<String>>
```

## 🚧 Roadmap Status

### ✅ Completado (Fase 1 - Semana 2)
- [x] Multi-timeframe AST nodes
- [x] Enhanced StrategyBuilder con métodos multi-TF
- [x] Multi-timeframe validation
- [x] Updated RandomGenerator
- [x] Basic SemanticConstraints structure
- [x] Comprehensive testing
- [x] Backward compatibility
- [x] Documentation

### 🟡 En Progreso (Next Phases)
- [ ] **Phase 3** (Semana 6-7): Semantic constraints implementation
  - [ ] Real correlation matrix calculation
  - [ ] Pearson correlation entre indicadores
  - [ ] Anti-correlation constraint enforcement
- [ ] **Phase 4** (Semana 8-11): Genetic algorithm implementation
  - [ ] Crossover y mutation operators
  - [ ] Multi-objective fitness (performance + diversity)
  - [ ] Advanced selection strategies

### 📋 Dependencies

- **Phase 2** (Semana 3-5): Strategy Converter Hub
  - [ ] Rhai â†' AST conversion
  - [ ] Multi-format support
- **Phase 4** (Semana 8-11): Backtest Engine
  - [ ] Performance fitness calculation
  - [ ] Real strategy evaluation

## 📚 API Reference

### Core Types

- `TimeframeCategory`: Current, Medium, High
- `StrategyAST`: Multi-timeframe strategy representation
- `IndicatorType`: Indicator with timeframe category
- `StrategyBuilder`: Fluent API for strategy construction
- `StrategyValidator`: Multi-TF validation with detailed reporting

### Builder Methods

- `add_entry_condition_with_timeframe()`: Add condition with specific timeframe
- `add_entry_conditions_multi_tf()`: Add multiple conditions with timeframes
- `golden_cross_multi_tf()`: Pre-built golden cross strategy
- `mean_reversion_multi_tf()`: Pre-built mean reversion strategy

### Generator Methods

- `generate_multi_timeframe()`: Generate single multi-TF strategy
- `generate_batch()`: Generate multiple strategies
- `generate_cross_timeframe_batch()`: Generate across different primary timeframes

## 🐛 Known Issues

1. **Genetic Algorithm**: Basic implementation, full algorithm in Phase 4
2. **Semantic Constraints**: Structure only, real correlation in Phase 3
3. **Performance Fitness**: Requires backtest engine (Phase 4)

## 🤝 Contributing

Para contribuir al desarrollo:

1. Seguir arquitectura Rust 2024 (sin mod.rs)
2. Mantener backward compatibility
3. Agregar tests comprehensivos
4. Documentar nuevas características

## 📞 Support

Para problemas específicos de multi-timeframe strategies:

- Usar `validate_detailed()` para debugging
- Verificar timeframe mapping con `strategy.timeframe_mapping()`
- Revisar consistencia con `strategy.display_summary()`

---

**Status**: ✅ **Ready for Phase 2 - Strategy Converter Hub**  
**Next**: Implementar Rhai â†' AST conversion con soporte multi-timeframe  
**Version**: 2.1.0-multi-timeframe  
**Last Updated**: October 2025
