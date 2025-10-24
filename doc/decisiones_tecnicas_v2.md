# 🎯 Decisiones Técnicas Clave - DarwinX v2.0

## 📋 Documento de Arquitectura Técnica
**Versión**: 2.0  
**Fecha**: Octubre 2025  
**Propósito**: Referencia de decisiones técnicas para desarrollo

---

## 🏗️ **Decisiones Fundamentales**

### 1. **Multi-Timeframe: Relativo Simple vs Absoluto**

#### ✅ **DECISIÓN**: Timeframe Relativo Simple
```rust
pub enum TimeframeCategory {
    Current,  // Timeframe principal
    Medium,   // 3-5x el principal  
    High,     // 12-24x el principal
}
```

#### **Justificación**:
- ✅ **Escalabilidad**: Misma estrategia funciona en 1m, 5m, 1h, etc.
- ✅ **Simplicidad**: Solo 3 categorías vs combinaciones infinitas
- ✅ **Realismo**: Mappings probados en trading real

#### **Mapping Concreto**:
| Principal | Current | Medium | High |
|-----------|---------|--------|------|
| 1m | 1m | 5m | 1h |
| 5m | 5m | 15m | 1h |
| 15m | 15m | 1h | 4h |
| 1h | 1h | 4h | 1d |

#### **Alternativas Rechazadas**:
- ❌ Timeframes absolutos (inflexible)
- ❌ Multipliers arbitrarios (1x, 7x, 13x - no son TF estándar)

---

### 2. **Estrategias Manuales: Rhai Scripts**

#### ✅ **DECISIÓN**: Rhai como DSL principal
```rust
// Sintaxis definitiva
strategy_timeframe("5m");

let ema_short = indicator("ema", [50], "current");
let ema_long = indicator("ema", [200], "medium");  
let rsi = indicator("rsi", [14], "current");

entry_rules("and", [
    crosses_above(ema_short, ema_long),
    rsi < 50.0
]);
```

#### **Justificación**:
- ✅ **Sandbox seguro**: No puede crashear el sistema
- ✅ **Hot reload**: Cambios sin recompilar
- ✅ **Sintaxis familiar**: Similar a Rust/JavaScript
- ✅ **Performance**: Compiled scripts

#### **Alternativas Rechazadas**:
- ❌ JSON DSL (muy verbose)
- ❌ YAML (limitado para lógica)
- ❌ Python embed (seguridad + performance)

---

### 3. **Generador Masivo: Semantic Constraints**

#### ✅ **DECISIÓN**: Constraints dinámicos basados en metadata
```rust
pub struct SemanticConstraints {
    pub max_per_category: HashMap<IndicatorCategory, usize>,
    pub max_similarity_score: f64,  // Correlation threshold
    pub max_complexity_score: f64,
}
```

#### **Justificación**:
- ✅ **Escalable**: Funciona con 10 o 1000 indicadores
- ✅ **Sin hardcoding**: Usa metadata automático
- ✅ **Basado en datos**: Similarity = correlación real de Pearson

#### **Similarity Calculation**:
```rust
// Pre-computar correlation matrix con datos históricos reales
let correlation = pearson_correlation(
    indicator_series_1,  // RSI(14) on BTCUSDT 1 year
    indicator_series_2   // Stochastic(14) on same data
); // Result: 0.85 (highly correlated)
```

#### **Alternativas Rechazadas**:
- ❌ Hardcoded combinations (no escala)
- ❌ Heuristic similarity (menos preciso)
- ❌ No constraints (strategies redundantes)

---

### 4. **Warmup Strategy: Realistic Limits**

#### ✅ **DECISIÓN**: Límites realistas por timeframe
```rust
impl WarmupLimits {
    pub fn for_timeframe(tf: TimeFrame) -> Self {
        match tf {
            TimeFrame::M1 => Self { max_days_download: 1 },
            TimeFrame::M5 => Self { max_days_download: 3 },
            TimeFrame::H1 => Self { max_days_download: 60 },   // 8.3 días para EMA(200)
            TimeFrame::D1 => Self { max_days_download: 365 },  // 200 días para EMA(200)
        }
    }
}
```

#### **Justificación**:
- ✅ **Realista**: EMA(200) 1d = 200 días, descargable
- ✅ **Práctico**: EMA(500) 1d = 500 días, muy costoso
- ✅ **Flexible**: Streaming para períodos cortos, download para largos

#### **Strategy Viability**:
| Estrategia | Indicador | Días Necesarios | Acción |
|------------|-----------|-----------------|--------|
| RSI(14) 5m | RSI | 0.05 | ✅ Stream |
| EMA(200) 1h | EMA | 8.3 | ✅ Download |
| EMA(200) 1d | EMA | 200 | ✅ Download |
| EMA(500) 1d | EMA | 500 | ❌ Not viable |

#### **Alternativas Rechazadas**:
- ❌ Warmup aproximado (no real)
- ❌ Sin límites (inviable para large periods)
- ❌ Solo streaming (insuficiente para long periods)

---

### 5. **Backtest Engine: Dual Mode**

#### ✅ **DECISIÓN**: Polars + Event-driven especializado
```rust
pub enum BacktestMode {
    VectorizedMassive,    // 10,000+ strategies, Polars
    EventDrivenRealistic, // 100 strategies, tick-by-tick
}
```

#### **Use Cases**:
- **Polars Mode**: Screening masivo, aproximación rápida
- **Event Mode**: Validation realista, ready para live

#### **Justificación**:
- ✅ **Speed vs Accuracy tradeoff**: Consciente y explícito
- ✅ **Workflow natural**: Screen → Validate → Deploy
- ✅ **Resource optimization**: Polars para throughput, Event para realism

#### **Performance Targets**:
| Mode | Input | Target | Use Case |
|------|-------|--------|----------|
| Polars | 10,000 strategies | < 60 min | Initial screening |
| Event | 100 strategies | < 30 min | Final validation |

#### **Alternativas Rechazadas**:
- ❌ Solo Polars (no realistic simulation)
- ❌ Solo Event-driven (too slow para mass screening)
- ❌ Unified engine (complex, compromises)

---

### 6. **Strategy Converter: Hub Central**

#### ✅ **DECISIÓN**: Converter como hub de todos los formatos
```rust
impl StrategyConverter {
    // Inputs: Multiple sources → StrategyAST
    fn from_rhai(script: &str) -> StrategyAST;
    fn from_json_dsl(json: &str) -> StrategyAST;
    fn from_freqtrade(config: &FreqtradeConfig) -> StrategyAST;
    
    // Outputs: StrategyAST → Executable formats
    fn to_polars_query(ast: &StrategyAST) -> PolarsBatchQuery;
    fn to_event_driven(ast: &StrategyAST) -> EventDrivenStrategy;
    fn to_rhai_runtime(ast: &StrategyAST) -> OptimizedRhaiScript;
}
```

#### **Justificación**:
- ✅ **Central hub**: Una sola responsabilidad, well-defined
- ✅ **Extensible**: Nuevos formats solo requieren new input/output
- ✅ **Testeable**: Clear interfaces para testing
- ✅ **StrategyAST**: Common intermediate representation

#### **Data Flow**:
```
Rhai Script → StrategyAST → Polars Query → Mass Backtest
               ↓
            Strategy Store
               ↓  
         StrategyAST → Event Strategy → Realistic Backtest
               ↓
         StrategyAST → Rhai Runtime → Live Trading
```

#### **Alternativas Rechazadas**:
- ❌ Direct conversions (Rhai → Polars) - no reusable
- ❌ Multiple converters - duplicated logic
- ❌ Format-specific engines - complex architecture

---

## 🔧 **Implementación Técnica**

### 1. **Multi-Timeframe Data Evaluation**

#### **Principio Fundamental**:
```rust
// Higher timeframes = vela cerrada anterior
// Current timeframe = vela actual
match indicator.timeframe_category {
    Current => get_current_value(timestamp),
    Medium | High => get_last_closed_value(timestamp), // ⚡ KEY INSIGHT
}
```

#### **Timeline Example (5m principal)**:
```
14:00 ■■■■ 15m closed → EMA = 42,150
14:05 ▓ 5m eval → RSI = 28.5, EMA_15m = 42,150 ✅
14:10 ▓ 5m eval → RSI = 31.2, EMA_15m = 42,150 ❌  
14:15 ■■■■ 15m closed → EMA = 42,200 (updated)
14:15 ▓ 5m eval → RSI = 29.8, EMA_15m = 42,200 ✅
```

### 2. **Rhai Built-in Functions**

#### **Core Functions**:
```rust
// Configuration
strategy_timeframe(tf: &str) -> ()
set_name(name: &str) -> ()

// Indicators  
indicator(name: &str, params: [f64], category: &str) -> IndicatorRef

// Conditions
crosses_above(a: IndicatorRef, b: IndicatorRef) -> Condition
crosses_below(a: IndicatorRef, b: IndicatorRef) -> Condition
price() -> PriceRef
volume() -> VolumeRef

// Rules
entry_rules(operator: &str, conditions: [Condition]) -> ()
exit_rules(operator: &str, conditions: [Condition]) -> ()
```

#### **Type System**:
```rust
// Rhai types mapping
pub struct IndicatorRef {
    name: String,
    params: Vec<f64>,
    timeframe_category: TimeframeCategory,
}

pub enum Condition {
    Comparison { left: Value, op: CompOp, right: Value },
    Cross { left: Value, direction: CrossDir, right: Value },
}
```

### 3. **Correlation Matrix Pre-computation**

#### **Reference Dataset**:
```rust
pub struct CorrelationDataset {
    symbol: &'static str,      // "BTCUSDT"
    timeframe: TimeFrame,      // H1
    period: Duration,          // 1 year
    candle_count: usize,       // 8760 candles
}
```

#### **Matrix Storage**:
```rust
// Cache structure
pub struct SimilarityCache {
    matrix: HashMap<(String, String), f64>,
    version: u32,
    computed_at: DateTime<Utc>,
}

// Persistent storage
// ~/.darwinx/similarity_cache.bin
// Format: MessagePack serialized HashMap
```

### 4. **Polars Query Generation**

#### **Strategy → Polars Pipeline**:
```rust
impl StrategyConverter {
    fn to_polars_query(&self, ast: &StrategyAST) -> PolarsBatchQuery {
        let mut query = LazyFrame::scan_parquet("data.parquet", Default::default());
        
        // Add indicator columns
        for indicator in ast.all_indicators() {
            query = query.with_column(
                self.indicator_to_polars_expr(indicator)
                    .alias(&format!("{}_{}", indicator.name, indicator.timeframe_category))
            );
        }
        
        // Add entry conditions
        let entry_expr = self.conditions_to_polars_expr(&ast.entry_rules);
        query = query.with_column(entry_expr.alias("entry_signal"));
        
        // Add exit conditions  
        let exit_expr = self.conditions_to_polars_expr(&ast.exit_rules);
        query = query.with_column(exit_expr.alias("exit_signal"));
        
        // Generate trades
        query = query.with_column(
            self.generate_trades_expr().alias("trades")
        );
        
        PolarsBatchQuery { query }
    }
}
```

---

## 🚦 **Error Handling Strategy**

### 1. **Rhai Parsing Errors**

```rust
#[derive(Debug, thiserror::Error)]
pub enum RhaiParseError {
    #[error("Syntax error at line {line}: {message}")]
    SyntaxError { line: usize, message: String },
    
    #[error("Unknown indicator '{name}'. Available: {available:?}")]
    UnknownIndicator { name: String, available: Vec<String> },
    
    #[error("Invalid timeframe category '{category}'. Use: current, medium, high")]
    InvalidTimeframeCategory { category: String },
    
    #[error("Logical contradiction: {description}")]
    LogicalContradiction { description: String },
}
```

### 2. **Warmup Planning Errors**

```rust
#[derive(Debug, thiserror::Error)]
pub enum WarmupError {
    #[error("Strategy not viable: {reason}")]
    StrategyNotViable { reason: String },
    
    #[error("Insufficient data: need {required} days, max {available}")]
    InsufficientData { required: u32, available: u32 },
    
    #[error("Exchange API error: {source}")]
    ExchangeError { source: ExchangeError },
}
```

### 3. **Backtest Errors**

```rust
#[derive(Debug, thiserror::Error)]
pub enum BacktestError {
    #[error("Strategy conversion failed: {reason}")]
    ConversionFailed { reason: String },
    
    #[error("Data loading failed: {source}")]
    DataLoadingFailed { source: DataError },
    
    #[error("Polars execution failed: {source}")]
    PolarsExecutionFailed { source: PolarsError },
}
```

---

## 📊 **Monitoring & Observability**

### 1. **Key Metrics**

```rust
// Performance metrics
pub struct SystemMetrics {
    pub rhai_parse_duration_ms: Histogram,
    pub strategy_conversion_duration_ms: Histogram,
    pub backtest_duration_seconds: Histogram,
    pub memory_usage_bytes: Gauge,
    pub active_strategies: Gauge,
}

// Business metrics  
pub struct BusinessMetrics {
    pub strategies_generated_total: Counter,
    pub backtests_completed_total: Counter,
    pub correlation_violations_total: Counter,
    pub warmup_failures_total: Counter,
}
```

### 2. **Logging Strategy**

```rust
// Structured logging
info!(
    strategy_id = %strategy.id,
    timeframe = %strategy.primary_timeframe,
    indicators = strategy.indicator_count(),
    complexity = strategy.complexity_score(),
    "Strategy backtest started"
);

warn!(
    indicator1 = %ind1.name,
    indicator2 = %ind2.name,
    correlation = %similarity_score,
    threshold = %threshold,
    "High correlation detected between indicators"
);
```

---

## 🔒 **Security Considerations**

### 1. **Rhai Sandbox**

```rust
// Restricted Rhai engine
let mut engine = rhai::Engine::new();

// Disable dangerous features
engine.disable_symbol("eval");
engine.disable_symbol("import");
engine.set_max_operations(10_000);  // Prevent infinite loops
engine.set_max_string_size(1_000);  // Prevent memory attacks

// Only allow safe modules
engine.register_global_module(create_trading_module());
```

### 2. **API Security**

```rust
// Rate limiting
pub struct RateLimiter {
    requests_per_minute: u32,
    max_concurrent_backtests: u32,
}

// Input validation
pub fn validate_strategy_ast(ast: &StrategyAST) -> Result<(), ValidationError> {
    // Max indicators
    ensure!(ast.indicator_count() <= 10, "Too many indicators");
    
    // Max complexity
    ensure!(ast.complexity_score() <= 100.0, "Strategy too complex");
    
    // Valid timeframes
    ensure!(ast.timeframes_are_valid(), "Invalid timeframe combination");
    
    Ok(())
}
```

---

## 🎯 **Next Steps - Implementation Order**

### **Immediate (Week 1)**:
1. ✅ Implement `MultiTimeframeContext` in `data/multi_timeframe.rs`
2. ✅ Update `StrategyAST` para timeframe categories
3. ✅ Write comprehensive tests

### **Short-term (Week 2-3)**:
1. ✅ Create `strategy-converter` crate structure  
2. ✅ Implement `RhaiParser::parse()`
3. ✅ Basic Rhai → AST conversion

### **Medium-term (Week 4-8)**:
1. ✅ Complete strategy converter hub
2. ✅ Implement semantic constraints
3. ✅ Build Polars backtest engine

La arquitectura está completamente definida y lista para implementación fase por fase.

---

**Prepared by**: DarwinX Architecture Team  
**Last Updated**: October 2025  
**Status**: ✅ **Ready for Implementation**
