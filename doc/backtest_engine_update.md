⚡ Especificación del Backtest Engine - Revisada
📋 Principios de Diseño
Responsabilidad Única:

El Backtest Engine solo simula la ejecución de trades
NO calcula indicadores (eso es responsabilidad de las estrategias)
NO evalúa condiciones (las estrategias ya vienen compiladas)
SÍ gestiona: órdenes, posiciones, balance, comisiones, slippage

Separación de Responsabilidades:
Strategy Generator → Compiled Strategy → Backtest Engine → Results
     ↑                      ↑                ↑
  (Genera AST)         (Lógica ejecutable)  (Simula trades)

🏗️ Arquitectura del Sistema
Input: Estrategia Compilada
rusttrait Strategy {
    fn name(&self) -> &str;
    fn should_enter_long(&self, candles: &[Candle], index: usize) -> bool;
    fn should_exit_long(&self, candles: &[Candle], index: usize) -> bool;
    fn should_enter_short(&self, candles: &[Candle], index: usize) -> bool;
    fn should_exit_short(&self, candles: &[Candle], index: usize) -> bool;
    fn position_size(&self, balance: f64, price: f64) -> f64;
}
Output: Resultado de Backtest
ruststruct BacktestResult {
    strategy_name: String,
    metrics: BacktestMetrics,
    trades: Vec<Trade>,
    equity_curve: Vec<EquityPoint>,
    metadata: BacktestMetadata,
}

🔥 Motor Polars (Vectorizado)
Características:

✅ Vectorizado: Procesa múltiples velas simultáneamente
✅ Lazy Evaluation: Optimización automática de operaciones
✅ Paralelización: Batch execution con Rayon
✅ Throughput: 10,000+ estrategias/hora
✅ Uso: Backtest masivo, análisis estadístico

Estructura:
rustcrates/backtest-engine/src/polars_engine/
├── vectorized.rs     // Motor principal vectorizado
├── parallel.rs       // Ejecución paralela con Rayon
└── optimizer.rs      // Optimización de queries
Flujo de Ejecución:
rustimpl PolarsBacktestEngine {
    async fn run_backtest(
        &self,
        strategy: &dyn Strategy,
        candles: &[Candle],
    ) -> Result<BacktestResult> {
        
        // 1. Convertir datos a DataFrame
        let df = self.candles_to_dataframe(candles)?;
        
        // 2. Evaluar estrategia para cada vela
        let signals = self.evaluate_strategy_vectorized(&df, strategy)?;
        
        // 3. Simular ejecución de trades
        let result = self.simulate_trades(strategy, candles, signals).await?;
        
        Ok(result)
    }
}

🎯 Motor Event-Driven (Simulación Realista)
Características:

✅ Tick-by-tick: Simulación granular paso a paso
✅ Order Book: Simulación realista de slippage
✅ Latencia: Simula delays reales del mercado
✅ Throughput: ~100 estrategias/hora
✅ Uso: Validación final, estrategias complejas

Estructura:
rustcrates/backtest-engine/src/event_driven/
├── engine.rs         // Motor principal event-driven
├── order_book.rs     // Simulación de order book
└── execution.rs      // Ejecución tick-by-tick
Flujo de Ejecución:
rustimpl EventDrivenEngine {
    async fn run_backtest(
        &self,
        strategy: &dyn Strategy,
        candles: &[Candle],
    ) -> Result<BacktestResult> {
        
        let mut state = BacktestState::new(self.config.initial_balance);
        
        // Procesar cada vela individualmente
        for (i, candle) in candles.iter().enumerate() {
            
            // 1. Actualizar estado del mercado
            self.update_market_state(&mut state, candle);
            
            // 2. Evaluar estrategia en este punto
            let entry_signal = strategy.should_enter_long(candles, i);
            let exit_signal = strategy.should_exit_long(candles, i);
            
            // 3. Procesar señales con order book
            if entry_signal {
                self.process_entry_order(&mut state, candle, strategy)?;
            }
            if exit_signal {
                self.process_exit_order(&mut state, candle)?;
            }
            
            // 4. Actualizar posiciones y equity
            self.update_positions(&mut state, candle)?;
        }
        
        Ok(self.finalize_backtest(state, strategy))
    }
}

📊 Sistema de Métricas
Estructura Modular:
rustcrates/backtest-engine/src/metrics/
├── returns.rs        // Sharpe, Sortino, Total Return
├── risk.rs           // Max DD, Calmar, VaR
└── statistics.rs     // Win Rate, Profit Factor
Métricas Calculadas:
ruststruct BacktestMetrics {
    // Returns
    total_return: f64,
    annualized_return: f64,
    cagr: f64,
    
    // Risk
    sharpe_ratio: f64,
    sortino_ratio: f64,
    max_drawdown: f64,
    calmar_ratio: f64,
    
    // Trading Stats
    total_trades: usize,
    win_rate: f64,
    profit_factor: f64,
    average_trade: f64,
}

🚀 Sistema de Batch Processing
Estructura:
rustcrates/backtest-engine/src/batch/
├── scheduler.rs      // Job scheduling y distribución
└── worker.rs         // Worker pool para paralelización
Funcionalidad:
rustimpl BatchScheduler {
    async fn run_batch(
        &self,
        strategies: Vec<Box<dyn Strategy>>,
        data: &[Candle],
        progress_callback: Option<ProgressCallback>,
    ) -> Result<Vec<BacktestResult>> {
        
        // Dividir trabajo entre workers
        let chunks = self.chunk_strategies(strategies);
        
        // Ejecutar en paralelo con Rayon
        let results: Vec<BacktestResult> = chunks
            .into_par_iter()
            .map(|chunk| self.process_chunk(chunk, data))
            .flatten()
            .collect::<Result<Vec<_>>>()?;
            
        Ok(results)
    }
}

💻 API Pública Unificada
Trait Principal:
rust#[async_trait::async_trait]
pub trait BacktestEngine {
    /// Ejecuta backtest individual
    async fn run_backtest(
        &self,
        strategy: &dyn Strategy,
        data: &[Candle],
        config: &BacktestConfig,
    ) -> Result<BacktestResult>;

    /// Ejecuta múltiples backtests en batch
    async fn run_batch(
        &self,
        strategies: Vec<Box<dyn Strategy>>,
        data: &[Candle],
        config: &BacktestConfig,
        progress_callback: Option<ProgressCallback>,
    ) -> Result<Vec<BacktestResult>>;
}
Implementaciones:
rust// Motor rápido para análisis masivo
pub struct PolarsBacktestEngine {
    config: BacktestConfig,
}

// Motor preciso para validación final
pub struct EventDrivenEngine {
    config: BacktestConfig,
    order_book: OrderBookSimulator,
}

🔧 Configuración del Sistema
BacktestConfig:
ruststruct BacktestConfig {
    initial_balance: f64,     // Balance inicial
    commission_rate: f64,     // Comisión por trade
    slippage_bps: f64,        // Slippage en basis points
    max_positions: usize,     // Máx posiciones simultáneas
    risk_per_trade: f64,      // Riesgo por trade (%)
}
```

### **Targets de Performance:**
```
┌─────────────────────────────────────────────────────────────┐
│                    PERFORMANCE TARGETS                     │
├─────────────────────────────────────┬───────────────────────┤
│ Operación                           │ Target                │
├─────────────────────────────────────┼───────────────────────┤
│ Backtest 1 estrategia (Polars)      │ < 1 segundo (100k velas) │
│ Backtest 1 estrategia (Event)       │ < 10 segundos (100k velas) │
│ Backtest masivo 10k (Polars)        │ < 60 minutos          │
│ Memory usage                        │ < 2GB por 100k velas │
│ CPU usage (batch)                   │ < 95%                 │
└─────────────────────────────────────┴───────────────────────┘

📁 Estructura de Archivos Completa
crates/backtest-engine/
├── Cargo.toml
├── src/
│   ├── lib.rs                    // API pública
│   ├── error.rs                  // Error handling
│   ├── types.rs                  // Tipos de datos
│   ├── metric.rs
│   ├── metrics/
│   │   ├── returns.rs           // Métricas de retorno
│   │   ├── risk.rs              // Métricas de riesgo
│   │   └── statistics.rs        // Estadísticas de trading
│   ├── polars_engine.rs
│   ├── polars_engine/
│   │   ├── vectorized.rs        // Motor vectorizado
│   │   ├── parallel.rs          // Ejecución paralela
│   │   └── optimizer.rs         // Optimización de queries
│   ├── event_driven.rs
│   ├── event_driven/
│   │   ├── engine.rs            // Motor event-driven
│   │   ├── order_book.rs        // Simulación order book
│   │   └── execution.rs         // Ejecución tick-by-tick
│   └── batch.rs
│   └── batch/
│       ├── scheduler.rs         // Job scheduling
│       └── worker.rs            // Worker pool
├── tests/
│   ├── integration_tests.rs     // Tests de integración
│   └── performance_tests.rs     // Benchmarks
└── benches/
    └── backtest_performance.rs  // Criterion benchmarks
```

---

## 🎯 **Flujo de Trabajo Completo**
```
1. Strategy Generator → Compile Strategy → CompiledStrategy
                                               ↓
2. CompiledStrategy + Historical Data → BacktestEngine
                                               ↓
3. BacktestEngine → Simulate Trades → BacktestResult
                                               ↓
4. BacktestResult → Calculate Metrics → Final Report


Responsabilidades Claras:

Strategy Generator: Genera y compila estrategias
Backtest Engine: Solo simula ejecución de trades
Metrics Calculator: Calcula estadísticas de performance
Batch Scheduler: Coordina ejecución masiva


✅ Principios de Diseño Finales

Single Responsibility: Cada componente tiene una función específica
Dependency Injection: Estrategias compiladas se inyectan
Performance First: Optimizado para throughput masivo
Testability: Interfaces limpias para testing
Extensibility: Fácil agregar nuevos motores o métricas

Esta especificación elimina completamente la dependencia del Registry en el Backtest Engine y establece responsabilidades claras para cada componente.