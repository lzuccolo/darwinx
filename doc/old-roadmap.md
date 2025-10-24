# 🗺️ DarwinX - Roadmap Completo Actualizado

**Fecha actualización:** 22 Octubre 2025  
**Repositorio:** https://github.com/lzuccolo/darwinx.git

---

## ✅ COMPLETADO (Semanas 1-6)

### **Fase 0: Setup Inicial** ✅ 100%
- [x] Proyecto compilable
- [x] Estructura de carpetas
- [x] CI/CD básico
- [x] Documentación inicial

### **Fase 1: Fundación** ✅ 100%
- [x] Core types (Candle, Signal, Position, Order, TimeFrame)
- [x] Traits (Strategy, MarketData, RiskManager, Exchange)
- [x] Proto files (5 archivos .proto completos)
- [x] **Indicadores básicos (16 indicadores)**
  - [x] Trend: sma, ema, wma, vwma
  - [x] Momentum: rsi, macd, stochastic, roc
  - [x] Volatility: atr, bollinger, keltner
  - [x] Volume: obv, mfi, vwap

### **Fase 1.5: Registry de Indicadores** ✅ 100% (NUEVO)
- [x] Sistema de auto-registro con `ctor`
- [x] Registry thread-safe con `parking_lot`
- [x] API pública: `get()`, `all_names()`, `by_category()`, `stats()`
- [x] Macro `register_indicator!()`
- [x] 41 tests pasando
- [x] Ejemplo funcional (`registry_demo`)

### **Fase 2: Storage & Data** ⚠️ 80%
- [x] Schema SQL (migrations)
- [x] Modelos (Strategy, BacktestResult, Trade)
- [x] Repositories (StrategyRepo, BacktestRepo)
- [x] CSV loader básico
- [x] Parquet loader básico
- [ ] Multi-timeframe context (PENDIENTE)
- [ ] Synchronizer (PENDIENTE)

### **Fase 3: Strategy Generator** 🚧 70%
- [x] AST nodes (StrategyAST, Condition, RuleSet)
- [x] IndicatorType dinámico (String + params)
- [x] RandomGenerator básico
- [x] Integración con Registry
- [ ] **ast/builder.rs** (ARREGLAR - usar nuevo IndicatorType)
- [ ] **ast/validator.rs** (ARREGLAR - usar nuevo IndicatorType)
- [ ] Genetic algorithm (PENDIENTE)
- [ ] Constraints avanzados (PENDIENTE)

---

## 🚧 EN PROGRESO (Semana actual)

### **Completar Fase 3: Strategy Generator**

**Tareas inmediatas:**

1. [ ] **Arreglar `ast/builder.rs`** (30 min)
   - Cambiar todas las referencias de enum a struct
   - `IndicatorType::Rsi { period }` → `IndicatorType::with_period("rsi", period)`
   
2. [ ] **Arreglar `ast/validator.rs`** (30 min)
   - Actualizar validaciones para estructura dinámica
   
3. [ ] **Completar tests de strategy-generator** (1 hora)
   - Test de generación con registry
   - Test de variedad de indicadores
   - Test de serialización JSON

4. [ ] **Implementar Genetic Algorithm** (Semana 7 - 8 horas)
   - Fitness function (Sharpe ratio)
   - Crossover operator
   - Mutation operator
   - Selection (tournament, roulette)
   - Population management
   - Convergence criteria

**Estimado:** 2-3 días

---

## 📅 ROADMAP PENDIENTE

### **Fase 4: Backtest Engine** (Semanas 8-9)

#### Semana 8: Polars Engine
- [ ] Vectorized backtest básico
- [ ] Order execution simulation
- [ ] Commission & slippage
- [ ] Position tracking
- [ ] Parallel executor (Rayon)
- [ ] Tests de backtest

**Entregable:** Motor vectorizado funcional

#### Semana 9: Metrics & Batch
- [ ] Métricas: Returns, Sharpe, Sortino
- [ ] Métricas: Max DD, Calmar, Win Rate
- [ ] Métricas: Profit Factor, Avg Trade
- [ ] Batch scheduler
- [ ] Worker pool
- [ ] Progress tracking
- [ ] Benchmarks (10k estrategias)

**Entregable:** Backtest masivo con métricas completas

---

### **Fase 5: gRPC Server** (Semanas 10-11)

#### Semana 10: Services Implementation
- [ ] StrategyService (CRUD + streaming)
- [ ] BacktestService (Run + Batch)
- [ ] OptimizerService
- [ ] DataService

#### Semana 11: Server Setup & Auth
- [ ] Server main (Tonic setup)
- [ ] Health check service
- [ ] Error handling middleware
- [ ] Logging middleware
- [ ] Auth básico (JWT)
- [ ] Config loading (TOML)
- [ ] Tests de integración
- [ ] Docker setup

**Entregable:** Servidor gRPC completo y desplegable

---

### **Fase 6: gRPC Client** (Semana 12)

- [ ] StrategyClient
- [ ] BacktestClient
- [ ] OptimizerClient
- [ ] LiveClient
- [ ] DataClient
- [ ] Connection management
- [ ] Retry logic
- [ ] Auth token handling
- [ ] Config loading
- [ ] Tests de integración

**Entregable:** Cliente gRPC completo

---

### **Fase 7: CLI Client** (Semana 13)

- [ ] Setup clap (args parsing)
- [ ] Command: generate
- [ ] Command: backtest
- [ ] Command: optimize
- [ ] Command: list
- [ ] Command: analyze
- [ ] Command: run (live)
- [ ] Output: tablas (comfy-table)
- [ ] Output: progress bars (indicatif)
- [ ] Output: charts ASCII
- [ ] Tests e2e

**Entregable:** CLI completo y usable

---

### **Fase 8: GUI Client** (Semanas 14-16)

#### Semana 14: Setup & Basic Views
- [ ] Setup GTK4 + Relm4
- [ ] Main window + navigation
- [ ] Models (Strategy, State)
- [ ] Messages (AppMsg)
- [ ] Services (gRPC integration)
- [ ] Sidebar component
- [ ] Generator view

#### Semana 15: Advanced Views
- [ ] Backtest view
- [ ] Strategy card widget
- [ ] Analysis view
- [ ] Live trading view
- [ ] Progress bars y spinners
- [ ] Metric cards
- [ ] CSS styling

#### Semana 16: Charts & Polish
- [ ] Chart widget (Plotters)
- [ ] Real-time updates (streaming)
- [ ] Error handling y dialogs
- [ ] Settings/Config view
- [ ] Keyboard shortcuts
- [ ] Testing manual (QA)
- [ ] Packaging (AppImage/Flatpak)

**Entregable:** GUI completa y pulida

---

## 🎯 MVP COMPLETO (Semana 16)

Al finalizar la Fase 8 tendrás:
- ✅ 16+ indicadores auto-registrados
- ✅ Generador de estrategias (random + genetic)
- ✅ Motor de backtest masivo (Polars)
- ✅ Servidor gRPC completo
- ✅ Cliente CLI funcional
- ✅ GUI nativa con GTK4
- ✅ Base de datos con persistencia
- ✅ Sistema completo end-to-end

---

## 🔮 FASES FUTURAS (Opcional - Semanas 17+)

### **Fase 9: Features Adicionales**

#### Converter (Semana 17)
- [ ] Intermediate → Rhai
- [ ] Intermediate → Rust
- [ ] Intermediate → Python/Freqtrade

#### Optimizer Avanzado (Semana 18)
- [ ] Grid search optimizado
- [ ] Genetic algorithm avanzado
- [ ] Bayesian optimization
- [ ] Walk-forward analysis

#### Live Trading (Semanas 19-20)
- [ ] Runner en vivo (Rhai)
- [ ] Exchange integration (Binance, Bybit)
- [ ] Risk management avanzado
- [ ] Paper trading mode
- [ ] WebSocket real-time
- [ ] Order management

#### Data Manager (Semana 21)
- [ ] Downloaders (Binance, Yahoo)
- [ ] Converters (CSV ↔ Parquet)
- [ ] Sync daemon
- [ ] Scheduler

---

## 📊 MÉTRICAS DE PROGRESO ACTUAL

| Módulo | Estado | Completado | Tests |
|--------|--------|------------|-------|
| **core** | ✅ | 100% | ✅ |
| **indicators** | ✅ | 100% | ✅ 41 tests |
| **data** | ⚠️ | 80% | ⚠️ |
| **strategy-store** | ✅ | 90% | ⚠️ |
| **strategy-generator** | 🚧 | 70% | 🚧 |
| **backtest-engine** | ❌ | 0% | ❌ |
| **api-proto** | ✅ | 100% | - |
| **api-server** | ❌ | 0% | ❌ |
| **api-client** | ❌ | 0% | ❌ |
| **cli-client** | ❌ | 0% | ❌ |
| **gui-client** | ❌ | 0% | ❌ |

**Progreso general:** ~30% del MVP completo

---

## 🎯 PRÓXIMAS ACCIONES INMEDIATAS

### **Esta semana:**

1. ✅ Arreglar `ast/builder.rs` y `ast/validator.rs`
2. ✅ Completar tests de strategy-generator
3. ✅ Implementar genetic algorithm

### **Próximas 2 semanas:**
4. ⏭️ Implementar Backtest Engine (Polars)
5. ⏭️ Implementar métricas completas
6. ⏭️ Batch execution paralelo

---

## 📝 NOTAS IMPORTANTES

### **Decisiones arquitectónicas tomadas:**
- ✅ Indicadores como funciones puras (máxima performance)
- ✅ Registry dinámico con auto-registro (sin hardcoding)
- ✅ IndicatorType como struct (name + params) - 100% dinámico
- ✅ gRPC para comunicación (type-safe, streaming)
- ✅ Polars para backtest vectorizado (10x más rápido)
- ✅ GTK4/Relm4 para GUI (nativo, rápido)

### **Performance Targets:**
- Backtest 1 estrategia: <1 segundo (100k velas)
- Backtest 10k estrategias: <60 minutos (paralelo 16 cores)
- Generación 1000 estrategias: <60 segundos
- gRPC latency: <5ms (p99)

---

## 🚀 COMANDOS ÚTILES

```bash
# Ver estado general
cd ~/shared/trading/src/darwinx
cargo test --workspace

# Test de indicadores
cd crates/indicators
cargo test
cargo run --example registry_demo

# Test de generator
cd crates/strategy-generator
cargo test

# Build completo
cd ~/shared/trading/src/darwinx
cargo build --workspace --release

# Ver cobertura
cargo tarpaulin --workspace --out Html
```

---

## 📚 RECURSOS

- **Documentación:** `docs/`
- **Ejemplos:** `crates/*/examples/`
- **Specs:** `tmp/darwinx_code_snapshot.txt`
- **GitHub:** https://github.com/lzuccolo/darwinx

---

**Última actualización:** 22 Octubre 2025  
**Estado:** Fase 3 en progreso - 70% completado  
**Próximo hito:** Completar Strategy Generator (3 días)