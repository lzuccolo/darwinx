# 🚀 Pipeline de Backtest Masivo - Especificación

## 🎯 Objetivo

Crear un pipeline completo que:
1. **Genere masivamente** 10,000-100,000 estrategias
2. **Backtestee con Polars vectorizado** (screening rápido)
3. **Seleccione las top 100** mejores estrategias
4. **Backtestee detalladamente** las top 100 con Event-Driven
5. **Genere reportes** comparativos

## 📊 Arquitectura del Pipeline

```
┌─────────────────────────────────────────────────────────────┐
│  FASE 1: Generación Masiva                                  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ RandomGenerator / GeneticGenerator                    │  │
│  │ → Genera 10,000-100,000 StrategyAST                  │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  FASE 2: Screening Masivo (Polars Vectorizado)             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ PolarsVectorizedBacktestEngine                        │  │
│  │ → Procesa todas las estrategias en batch             │  │
│  │ → Usa DataFrame de Polars para operaciones          │  │
│  │ → Calcula métricas básicas (Sharpe, Sortino, etc.)  │  │
│  │ → Throughput: 10,000+ estrategias/minuto            │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  FASE 3: Ranking y Selección                                │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ StrategyRanker                                       │  │
│  │ → Scoring compuesto (Sharpe, Sortino, Profit Factor)│  │
│  │ → Filtros de calidad (min trades, win rate, etc.)  │  │
│  │ → Selecciona top 100 estrategias                   │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  FASE 4: Backtest Detallado (Event-Driven)                  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ EventDrivenBacktestEngine                            │  │
│  │ → Backtest realista tick-by-tick                     │  │
│  │ → Métricas detalladas y precisas                     │  │
│  │ → Simulación completa de ejecución                  │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  FASE 5: Reporte y Análisis                                 │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ ReportGenerator                                      │  │
│  │ → Comparación de métricas                           │  │
│  │ → Análisis de correlación entre estrategias         │  │
│  │ → Exportación a JSON/CSV                             │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## 🔧 Componentes Técnicos

### 1. PolarsVectorizedBacktestEngine

**Objetivo**: Backtest masivo usando operaciones vectorizadas de Polars

**Implementación**:
```rust
pub struct PolarsVectorizedBacktestEngine;

impl PolarsVectorizedBacktestEngine {
    /// Ejecuta backtest masivo para múltiples estrategias
    pub async fn run_massive_backtest(
        &self,
        strategies: Vec<StrategyAST>,
        data: DataFrame,  // Datos en Polars DataFrame
        config: &BacktestConfig,
    ) -> Result<Vec<BacktestResult>, BacktestError> {
        // 1. Convertir estrategias a expresiones de Polars
        let expressions = strategies.iter()
            .map(|s| self.strategy_to_polars_expr(s))
            .collect();
        
        // 2. Calcular señales vectorizadas para todas las estrategias
        let signals_df = data
            .lazy()
            .with_columns(expressions)
            .collect()?;
        
        // 3. Calcular trades y métricas vectorizadas
        let results = self.calculate_metrics_vectorized(signals_df)?;
        
        Ok(results)
    }
    
    /// Convierte StrategyAST a expresión de Polars
    fn strategy_to_polars_expr(&self, strategy: &StrategyAST) -> Expr {
        // Convertir condiciones de entrada/salida a expresiones Polars
        // Ejemplo: RSI < 30 → col("rsi").lt(30.0)
    }
}
```

**Características**:
- Procesa todas las estrategias en un solo DataFrame
- Usa expresiones de Polars para señales
- Cálculo vectorizado de métricas
- Paralelización automática de Polars

### 2. StrategyRanker

**Objetivo**: Ranking y selección de mejores estrategias

**Implementación**:
```rust
pub struct StrategyRanker {
    weights: RankingWeights,
    filters: QualityFilters,
}

pub struct RankingWeights {
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,
    pub profit_factor: f64,
    pub total_return: f64,
    pub max_drawdown: f64,  // Negativo (menor es mejor)
}

pub struct QualityFilters {
    pub min_trades: usize,
    pub min_win_rate: f64,
    pub min_sharpe: f64,
    pub max_drawdown: f64,
}

impl StrategyRanker {
    /// Calcula score compuesto para una estrategia
    pub fn calculate_score(&self, result: &BacktestResult) -> f64 {
        let metrics = &result.metrics;
        
        // Normalizar métricas (0-1)
        let sharpe_norm = self.normalize_sharpe(metrics.sharpe_ratio);
        let sortino_norm = self.normalize_sortino(metrics.sortino_ratio);
        let pf_norm = self.normalize_profit_factor(metrics.profit_factor);
        let return_norm = self.normalize_return(metrics.total_return);
        let dd_norm = 1.0 - self.normalize_drawdown(metrics.max_drawdown);
        
        // Score ponderado
        self.weights.sharpe_ratio * sharpe_norm +
        self.weights.sortino_ratio * sortino_norm +
        self.weights.profit_factor * pf_norm +
        self.weights.total_return * return_norm +
        self.weights.max_drawdown * dd_norm
    }
    
    /// Filtra y selecciona top N estrategias
    pub fn select_top_n(
        &self,
        results: Vec<BacktestResult>,
        n: usize,
    ) -> Vec<BacktestResult> {
        results
            .into_iter()
            .filter(|r| self.passes_filters(r))
            .map(|r| {
                let score = self.calculate_score(&r);
                (r, score)
            })
            .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap())
            .take(n)
            .map(|(r, _)| r)
            .collect()
    }
}
```

### 3. MassiveBacktestPipeline

**Objetivo**: Orquestar todo el pipeline

**Implementación**:
```rust
pub struct MassiveBacktestPipeline {
    generator: RandomGenerator,
    polars_engine: PolarsVectorizedBacktestEngine,
    event_engine: EventDrivenBacktestEngine,
    ranker: StrategyRanker,
}

impl MassiveBacktestPipeline {
    /// Ejecuta el pipeline completo
    pub async fn run(
        &self,
        num_strategies: usize,
        top_n: usize,
        data_path: &str,
    ) -> Result<PipelineResult, BacktestError> {
        // FASE 1: Generación masiva
        println!("Generando {} estrategias...", num_strategies);
        let strategies = self.generator.generate_batch(num_strategies);
        
        // FASE 2: Cargar datos y convertir a Polars DataFrame
        let candles = CsvLoader::load(data_path)?;
        let df = self.candles_to_dataframe(&candles)?;
        
        // FASE 3: Screening masivo con Polars
        println!("Ejecutando backtest masivo con Polars...");
        let screening_results = self.polars_engine
            .run_massive_backtest(strategies, df, &BacktestConfig::default())
            .await?;
        
        // FASE 4: Ranking y selección
        println!("Seleccionando top {} estrategias...", top_n);
        let top_strategies = self.ranker.select_top_n(screening_results, top_n);
        
        // FASE 5: Backtest detallado con Event-Driven
        println!("Ejecutando backtest detallado para top {}...", top_n);
        let detailed_results = self.run_detailed_backtest(top_strategies, &candles).await?;
        
        Ok(PipelineResult {
            total_generated: num_strategies,
            top_selected: top_n,
            screening_results: screening_results.len(),
            detailed_results,
        })
    }
}
```

## 📊 Métricas de Performance Esperadas

| Fase | Estrategias | Tiempo Estimado | Throughput |
|------|-------------|-----------------|------------|
| Generación | 10,000 | ~1 min | 10K/min |
| Screening Polars | 10,000 | ~5-10 min | 1K-2K/min |
| Ranking | 10,000 | ~1 seg | 10K/seg |
| Event-Driven | 100 | ~10-30 min | 3-10/min |

**Total para 10K estrategias**: ~20-40 minutos

## 🎯 Casos de Uso

### Caso 1: Screening Inicial
```rust
let pipeline = MassiveBacktestPipeline::new();
let result = pipeline.run(
    10_000,  // Generar 10K estrategias
    100,     // Seleccionar top 100
    "data/btcusdt_1h.csv"
).await?;
```

### Caso 2: Screening Masivo
```rust
let result = pipeline.run(
    100_000, // Generar 100K estrategias
    100,     // Seleccionar top 100
    "data/btcusdt_1h.csv"
).await?;
```

## 🧬 Evolución Genética (Implementado)

El pipeline ahora incluye evolución genética opcional después del backtest inicial:

### Flujo con Evolución

```
FASE 1-5: Generación y Backtest Inicial
    ↓
FASE 6: Evolución Genética (opcional con --evolve)
    ├─ Selecciona top estrategias del backtest inicial
    ├─ Crea función de fitness basada en métricas
    ├─ Evoluciona estrategias (crossover + mutación)
    └─ Backtestea estrategias evolucionadas
    ↓
FASE 7: Re-filtrado y Re-ranqueo
    ├─ Combina resultados originales + evolucionados
    ├─ Re-filtra y re-ranquea todas las estrategias
    └─ Selecciona mejores finales
    ↓
FASE 8: Guardado en SQLite
```

### Función de Fitness

La función de fitness combina múltiples métricas normalizadas:

```rust
fitness = w1 * Sharpe_norm + 
          w2 * Sortino_norm + 
          w3 * ProfitFactor_norm + 
          w4 * Return_norm + 
          w5 * (1 - Drawdown_norm)
```

### Configuración

- `--evolve N`: Número de generaciones
- `--evolve-population SIZE`: Tamaño de población (default: 100)
- `--evolve-mutation-rate RATE`: Tasa de mutación (default: 0.1)
- `--evolve-elite-size SIZE`: Tamaño de elite (default: 10)

### Retroalimentación con SQLite

- `--load-best N`: Carga N mejores estrategias históricas como población inicial
- Las estrategias evolucionadas se guardan en SQLite para futuras ejecuciones
- Sistema de deduplicación evita guardar estrategias idénticas

## 📝 Estado de Implementación

1. ✅ **PolarsVectorizedBacktestEngine** - IMPLEMENTADO
2. ✅ **StrategyRanker** - IMPLEMENTADO (integrado en CLI)
3. ✅ **MassiveBacktestPipeline** - IMPLEMENTADO (CLI `massive_backtest`)
4. ✅ **Evolución Genética** - IMPLEMENTADO
5. ✅ **Persistencia SQLite** - IMPLEMENTADO
6. ⏳ **Tests de performance** - Pendiente
7. ✅ **Documentación básica** - COMPLETADO

## 🚀 Próximos Pasos

1. **Tests de performance** (validar throughput)
2. **Optimización de batch processing** para 100K+ estrategias
3. **Event-Driven backtest detallado** para top estrategias (opcional)
4. **Análisis de correlación** entre estrategias

