# 🗺️ DarwinX Trading Bot - Roadmap v2.1 (Risk Management Emergente)

## 📊 Estado Actual + Nuevas Decisiones (Octubre 2025)

### ✅ **COMPLETADO (50% del proyecto total)**
| Crate | Progreso | Estado | LOC | Funcionalidad |
|-------|----------|--------|-----|---------------|
| **Core** | 100% ✅ | Production Ready | 1,500 | Types, traits fundamentales |
| **Indicators** | 100% ✅ | Production Ready | 2,000 | 16 indicadores + registry dinámico |
| **API Proto** | 100% ✅ | Production Ready | 800 | gRPC definitions |
| **Strategy Store** | 95% ✅ | Nearly Complete | 2,500 | DB models + repositories |
| **Strategy Generator** | 100% ✅ | Production Ready | 3,500 | Random + Genetic + AST |

### ⚠️ **PARCIALMENTE COMPLETADO**
| Crate | Progreso | Estado | Falta Implementar |
|-------|----------|--------|-------------------|
| **Data** | 40% ⚠️ | Needs Multi-TF | `multi_timeframe.rs` (solo stub) |

### 🎯 **DECISIÓN CRÍTICA: Risk Management Emergente**

#### **Cambio Fundamental**:
```
❌ ANTES: Risk management fijo en screening masivo
✅ AHORA: Pure signal analysis → Emergent risk derivation
```

#### **Pipeline Actualizado**:
```
1. Pure Signal Screening (10,000 strategies, SIN risk management)
2. Top Performers Selection (100-200 best pure signals)  
3. Emergent Risk Derivation (MAE/MFE analysis → optimal risk)
4. Realistic Validation (Event-driven con emergent risk)
5. Production Deployment (Ready-to-trade strategies)
```

---

## 🚀 **ROADMAP v2.1 - Emergent Risk Integration**

### **FASE 1: Multi-Timeframe Foundation** 
**⏱️ Duración**: 2 semanas  
**🎯 Objetivo**: Completar soporte multi-timeframe

#### Semana 1: Data Multi-Timeframe Core
```rust
crates/data/src/multi_timeframe/
├── context.rs           ✨ NEW - Multi-TF context manager
├── synchronizer.rs      ✨ NEW - Timeframe synchronization  
├── cache.rs            ✨ NEW - Multi-TF data cache
└── alignment.rs        ✨ NEW - Temporal alignment
```

**Deliverables Semana 1**:
- [ ] `MultiTimeframeContext` - Manage multiple timeframe data
- [ ] `TimeframeSynchronizer` - Sync diferentes TFs con forward-fill
- [ ] `MultiTimeframeDataCache` - Cache eficiente por TF
- [ ] **Testing**: Unit tests para sincronización

#### Semana 2: Strategy AST Multi-Timeframe  
```rust
crates/strategy-generator/src/ast/
├── nodes.rs            🔧 UPDATE - Add TimeframeCategory
├── builder.rs          🔧 UPDATE - Multi-TF builder methods
└── validator.rs        🔧 UPDATE - Multi-TF validation
```

**Deliverables Semana 2**:
- [ ] `TimeframeCategory` enum (Current/Medium/High)
- [ ] `IndicatorType` con timeframe_category field
- [ ] `StrategyBuilder` con métodos multi-TF
- [ ] **Testing**: Multi-TF strategy creation

### **FASE 2: Strategy Converter Hub**
**⏱️ Duración**: 3 semanas  
**🎯 Objetivo**: Central conversion hub

#### Semana 3: Converter Foundation
```rust
crates/strategy-converter/
├── Cargo.toml          ✨ NEW
└── src/
    ├── lib.rs          ✨ NEW - Main converter interface
    ├── formats.rs      ✨ NEW - Format definitions
    └── error.rs        ✨ NEW - Conversion errors
```

#### Semana 4: Rhai Parser Implementation
```rust
crates/strategy-converter/src/inputs/
├── rhai_parser.rs      ✨ NEW - Rhai script parser
├── rhai_validator.rs   ✨ NEW - Rhai semantic validation
└── rhai_functions.rs   ✨ NEW - Built-in functions
```

#### Semana 5: Output Formats
```rust
crates/strategy-converter/src/outputs/
├── polars_query.rs     ✨ NEW - AST → Polars query
├── event_driven.rs     ✨ NEW - AST → Event strategy
└── rhai_runtime.rs     ✨ NEW - AST → Optimized Rhai
```

### **FASE 2.5: Risk Management Emergente** ⭐ **NUEVA FASE**
**⏱️ Duración**: 2 semanas  
**🎯 Objetivo**: Pure signal analysis + emergent risk derivation

#### Semana 5.5: Pure Signal Analysis Engine
```rust
crates/pure-signal-analysis/
├── Cargo.toml          ✨ NEW
└── src/
    ├── lib.rs          ✨ NEW
    ├── signal_analyzer.rs    ✨ NEW - Pure signal analysis
    ├── excursion_calc.rs     ✨ NEW - MAE/MFE calculation
    ├── quality_metrics.rs    ✨ NEW - Signal quality scoring
    └── screening_engine.rs   ✨ NEW - Mass screening
```

**Deliverables Semana 5.5**:
- [ ] `PureSignalAnalysis` - Metrics de calidad sin risk management
- [ ] `ExcursionCalculator` - MAE/MFE calculation engine
- [ ] `SignalQualityScorer` - Composite quality metrics
- [ ] `MassScreeningEngine` - Screen 10,000 strategies < 30 min
- [ ] **Testing**: Validate signal quality metrics

#### Semana 6.5: Emergent Risk Derivation
```rust
crates/emergent-risk/
├── Cargo.toml          ✨ NEW
└── src/
    ├── lib.rs          ✨ NEW
    ├── risk_analyzer.rs      ✨ NEW - Statistical analysis engine
    ├── mae_analysis.rs       ✨ NEW - MAE-based stop loss derivation
    ├── mfe_analysis.rs       ✨ NEW - MFE-based take profit derivation
    ├── position_sizing.rs    ✨ NEW - Frequency-based position sizing
    └── asset_aware.rs        ✨ NEW - Asset class adaptations
```

**Deliverables Semana 6.5**:
- [ ] `RiskProfileAnalyzer` - Statistical derivation engine
- [ ] `MAEAnalysis` - Stop loss derivation from observed data
- [ ] `MFEAnalysis` - Take profit derivation from observed data
- [ ] `AssetAwareRisk` - Asset class specific adjustments
- [ ] **Testing**: Validate emergent risk vs fixed risk performance
- [ ] **Integration**: Complete Pure → Emergent pipeline

### **FASE 3: Semantic Constraints**
**⏱️ Duración**: 2 semanas  
**🎯 Objetivo**: Intelligent generator con correlation avoidance

#### Semana 6: Correlation Calculator (moved from week 6)
```rust
crates/strategy-converter/src/similarity/
├── correlation.rs      ✨ NEW - Pearson correlation calculator
├── matrix_builder.rs   ✨ NEW - Pre-compute correlation matrix
└── cache.rs           ✨ NEW - Similarity cache manager
```

#### Semana 7: Semantic Constraints Integration
```rust
crates/strategy-generator/src/
├── constraints.rs      🔧 UPDATE - Add SemanticConstraints
└── generator/
    ├── random.rs       🔧 UPDATE - Use constraints
    └── genetic.rs      🔧 UPDATE - Use constraints
```

### **FASE 4: Backtest Engine Dual Mode**
**⏱️ Duración**: 4 semanas  
**🎯 Objetivo**: Polars (pure signals) + Event-driven (realistic)

#### Semana 8-9: Pure Signal Backtest Engine
```rust
crates/backtest-engine/
├── Cargo.toml          ✨ NEW
└── src/
    ├── lib.rs          ✨ NEW
    ├── pure_signal_engine.rs ✨ NEW - Pure signal evaluation
    ├── polars_engine.rs      ✨ NEW - Vectorized mass backtest
    └── polars_engine/
        ├── vectorized.rs ✨ NEW - Polars operations
        ├── parallel.rs   ✨ NEW - Rayon parallelization  
        └── metrics.rs    ✨ NEW - Pure signal metrics
```

**Deliverables Semana 8**:
- [ ] `PureSignalBacktest` - Evaluate signals without risk management
- [ ] `PolarsBacktestEngine` basic structure para batch processing
- [ ] Pure signal metrics calculation (win rate, profit factor, excursions)
- [ ] **Testing**: Pure signal accuracy validation

**Deliverables Semana 9**:
- [ ] Batch backtest de 10,000+ strategies en paralelo
- [ ] Memory-efficient chunking para mass processing
- [ ] Progress reporting para long-running batch jobs
- [ ] **Performance**: 10,000 pure strategies en < 30 min

#### Semana 10-11: Event-Driven Realistic Engine
```rust
crates/backtest-engine/src/
├── event_driven.rs     ✨ NEW - Event-by-event simulation
└── event_driven/
    ├── engine.rs       ✨ NEW - Main simulation loop
    ├── order_book.rs   ✨ NEW - Order book simulation
    ├── execution.rs    ✨ NEW - Realistic execution
    ├── slippage.rs     ✨ NEW - Slippage modeling
    └── emergent_risk.rs ✨ NEW - Emergent risk integration
```

**Deliverables Semana 10**:
- [ ] `EventDrivenEngine` - Tick-by-tick realistic simulation
- [ ] Integration con emergent risk management
- [ ] Order execution simulation con fees y slippage
- [ ] **Testing**: Event engine vs pure signal comparison

**Deliverables Semana 11**:
- [ ] Complete emergent risk pipeline validation
- [ ] Realistic execution metrics y analysis
- [ ] Production-ready strategy identification
- [ ] **Performance**: 100 strategies con emergent risk < 30 min

### **FASE 5: Warmup Strategy & Asset Classes**
**⏱️ Duración**: 2 semanas  
**🎯 Objetivo**: Live trading preparation + asset-aware risk

#### Semana 12: Asset-Aware Risk Management
```rust
crates/emergent-risk/src/
├── asset_classes.rs    ✨ NEW - Asset class definitions
├── volatility_adjust.rs ✨ NEW - Volatility-based adjustments
└── cross_asset.rs      ✨ NEW - Cross-asset analysis
```

**Deliverables Semana 12**:
- [ ] `AssetClass` definitions (BTC, SP500, Forex, etc.)
- [ ] Volatility-adjusted risk management
- [ ] Cross-asset strategy performance analysis
- [ ] **Testing**: Same strategy different assets

#### Semana 13: Warmup Planning & Data Download
```rust
crates/runner-live/
├── Cargo.toml          ✨ NEW
└── src/
    ├── warmup/
    │   ├── planner.rs  ✨ NEW - Warmup planning logic
    │   ├── limits.rs   ✨ NEW - Asset-specific timeframe limits
    │   └── validator.rs ✨ NEW - Strategy viability
    └── data/
        ├── downloader.rs ✨ NEW - Historical data download
        └── cache.rs     ✨ NEW - Warmup data cache
```

**Deliverables Semana 13**:
- [ ] `WarmupPlanner` - Asset-aware warmup requirements
- [ ] Historical data downloader para multiple exchanges
- [ ] Strategy viability validation
- [ ] **Integration**: Complete live trading preparation pipeline

### **FASE 6: API Layer**
**⏱️ Duración**: 2 semanas  
**🎯 Objetivo**: Complete gRPC implementation

#### Semana 14: gRPC Server Implementation
```rust
crates/api-server/
├── Cargo.toml          ✨ NEW
└── src/
    ├── main.rs         ✨ NEW - Server startup
    ├── server.rs       ✨ NEW - Main server
    └── services/
        ├── strategy_service.rs  ✨ NEW - Strategy CRUD + Rhai + Pure analysis
        ├── backtest_service.rs  ✨ NEW - Pure + Emergent + Realistic modes
        ├── emergent_service.rs  ✨ NEW - Emergent risk derivation service
        └── live_service.rs      ✨ NEW - Live trading with emergent risk
```

#### Semana 15: gRPC Client Library
```rust
crates/api-client/
├── Cargo.toml          ✨ NEW
└── src/
    ├── lib.rs          ✨ NEW - Client library
    └── services/
        ├── strategy_client.rs   ✨ NEW - Strategy service client
        ├── backtest_client.rs   ✨ NEW - All backtest modes
        ├── emergent_client.rs   ✨ NEW - Emergent risk client
        └── live_client.rs       ✨ NEW - Live trading client
```

### **FASE 7: CLI Client**
**⏱️ Duración**: 1 semana  
**🎯 Objetivo**: Complete command-line interface

#### Semana 16: CLI Implementation
```rust
crates/cli-client/
├── Cargo.toml          ✨ NEW
└── src/
    ├── main.rs         ✨ NEW - CLI entry point
    ├── commands/
    │   ├── generate.rs ✨ NEW - Generate pure strategies
    │   ├── screen.rs   ✨ NEW - Pure signal screening
    │   ├── derive.rs   ✨ NEW - Derive emergent risk
    │   ├── backtest.rs ✨ NEW - All backtest modes
    │   ├── rhai.rs     ✨ NEW - Rhai script commands
    │   └── live.rs     ✨ NEW - Live trading commands
    └── output/
        ├── table.rs    ✨ NEW - Table formatting
        ├── charts.rs   ✨ NEW - ASCII charts
        └── progress.rs ✨ NEW - Progress bars
```

### **FASE 8: GUI Client**
**⏱️ Duración**: 4 semanas  
**🎯 Objetivo**: Native GUI con emergent risk visualization

#### Semana 17-18: GUI Foundation + Pure Signal Views
```rust
crates/gui-client/
├── Cargo.toml          ✨ NEW
└── src/
    ├── main.rs         ✨ NEW - GUI entry point
    ├── app.rs          ✨ NEW - Main app structure
    └── components/
        ├── sidebar.rs           ✨ NEW - Navigation sidebar
        ├── generator_view.rs    ✨ NEW - Strategy generator view
        ├── screening_view.rs    ✨ NEW - Pure signal screening view
        └── signal_analysis.rs   ✨ NEW - Signal quality visualization
```

#### Semana 19-20: Emergent Risk Visualization + Advanced Views
```rust
crates/gui-client/src/components/
├── rhai_editor.rs      ✨ NEW - Rhai script editor
├── emergent_risk.rs    ✨ NEW - MAE/MFE analysis visualization
├── backtest_view.rs    ✨ NEW - Multi-mode backtest interface
├── risk_derivation.rs  ✨ NEW - Risk derivation visualization
├── results_view.rs     ✨ NEW - Results comparison (pure vs emergent)
└── live_view.rs        ✨ NEW - Live trading dashboard
```

---

## 🎯 **MILESTONES ACTUALIZADOS**

### **Milestone 1**: Multi-Timeframe Ready (End Week 2)
- [ ] ✅ Multi-timeframe data loading functional
- [ ] ✅ Strategy AST supports timeframe categories  
- [ ] **Success**: Create EMA(200) 1h + RSI(14) 5m strategy

### **Milestone 2**: Strategy Converter Complete (End Week 5)
- [ ] ✅ Rhai scripts parse to StrategyAST
- [ ] ✅ Multi-format conversion working
- [ ] **Success**: Parse 100+ different Rhai scripts

### **Milestone 2.5**: 🌟 **Pure Signal Analysis** (End Week 5.5)
- [ ] ✅ Pure signal analysis engine functional
- [ ] ✅ Screen 10,000 strategies < 30 min
- [ ] ✅ MAE/MFE calculation accurate
- [ ] **Success**: Identify top 100 pure signal performers

### **Milestone 2.6**: 🌟 **Emergent Risk Derivation** (End Week 6.5)
- [ ] ✅ Emergent risk derivation from MAE/MFE
- [ ] ✅ Asset-aware risk adjustments
- [ ] ✅ Complete Pure → Emergent pipeline
- [ ] **Success**: Derive risk management > 80% confidence

### **Milestone 3**: Semantic Constraints (End Week 7)
- [ ] ✅ Correlation-based constraint system
- [ ] ✅ Generate 10,000 diverse strategies
- [ ] **Success**: Max 70% correlation between indicators

### **Milestone 4**: Dual Backtest Engine (End Week 11)
- [ ] ✅ Pure signal engine: 10,000 strategies < 30 min
- [ ] ✅ Event-driven engine: 100 strategies < 30 min
- [ ] ✅ Emergent risk integration working
- [ ] **Success**: Complete Pure → Emergent → Validation pipeline

### **Milestone 5**: Asset-Aware & Live Ready (End Week 13)
- [ ] ✅ Asset class definitions complete
- [ ] ✅ Warmup planning functional
- [ ] ✅ Live trading preparation ready
- [ ] **Success**: Multi-asset strategy deployment ready

### **Milestone 6**: Complete API (End Week 15)
- [ ] ✅ All gRPC services implemented
- [ ] ✅ Emergent risk services functional
- [ ] ✅ Client library complete
- [ ] **Success**: End-to-end API workflows

### **Milestone 7**: CLI Complete (End Week 16)
- [ ] ✅ All operations available via CLI
- [ ] ✅ Emergent risk derivation via CLI
- [ ] **Success**: Complete workflows via command line

### **Milestone 8**: GUI Complete (End Week 20)
- [ ] ✅ Native GUI with all functionality
- [ ] ✅ Emergent risk visualization
- [ ] ✅ MAE/MFE analysis charts
- [ ] **Success**: Complete visual emergent risk workflow

---

## 📊 **SUCCESS METRICS EMERGENTE**

### **Pure Signal Analysis**:
| Métrica | Target | Measurement |
|---------|--------|-------------|
| **Screening Speed** | 10,000 strategies < 30 min | Benchmark timing |
| **Signal Quality** | Top 100 strategies > 6.0 quality score | Quality metrics |
| **Signal Coverage** | >90% strategies with 20+ signals | Signal frequency |

### **Emergent Risk Derivation**:
| Métrica | Target | Measurement |
|---------|--------|-------------|
| **Derivation Confidence** | >80% statistical confidence | MAE/MFE analysis |
| **Performance Improvement** | >20% better than fixed risk | A/B comparison |
| **Asset Adaptation** | Risk adjusts to asset volatility | Cross-asset validation |

### **Production Pipeline**:
| Métrica | Target | Measurement |
|---------|--------|-------------|
| **End-to-End Time** | Complete pipeline < 2 hours | Full workflow timing |
| **Production Ready** | 10-50 strategies per asset | Final filtering |
| **Cross-Asset Success** | Same strategy works on 3+ assets | Multi-asset testing |

---

## 🚨 **RISK MITIGATION ACTUALIZADA**

### **High-Risk Areas**:

| Risk | Impact | Mitigation Strategy |
|------|--------|-------------------|
| **MAE/MFE Calculation Accuracy** | 🔴 Critical | Extensive backtesting validation, multiple timeframes |
| **Emergent Risk Overfitting** | 🔴 High | Large sample sizes, out-of-sample validation |
| **Pure Signal Quality Metrics** | 🟡 Medium | Multiple quality metrics, cross-validation |
| **Asset Class Differences** | 🟡 Medium | Separate analysis per asset, volatility adjustments |

### **Contingency Plans**:

**If Emergent Risk fails**:
- Fallback: Asset-aware fixed risk profiles
- Timeline impact: -1 week (simpler approach)

**If Pure Signal analysis is insufficient**:
- Fallback: Simplified screening with basic metrics
- Timeline impact: No impact, less sophisticated analysis

**If Cross-Asset performance poor**:
- Fallback: Asset-specific strategy development
- Timeline impact: +2 weeks for separate pipelines

---

## 🏁 **TIMELINE SUMMARY ACTUALIZADO**

| Phase | Weeks | Key Deliverables |
|-------|-------|------------------|
| **1. Multi-TF Foundation** | 1-2 | Multi-timeframe data + AST |
| **2. Strategy Converter** | 3-5 | Rhai parsing + format conversion |
| **2.5. 🌟 Emergent Risk** | 5.5-6.5 | Pure signal analysis + emergent risk derivation |
| **3. Semantic Constraints** | 6-7 | Correlation-aware generator |
| **4. Backtest Engine** | 8-11 | Pure signal + Event-driven + Emergent integration |
| **5. Asset-Aware & Warmup** | 12-13 | Asset classes + live trading prep |
| **6. API Layer** | 14-15 | Complete gRPC + emergent services |
| **7. CLI Client** | 16 | Command-line interface |
| **8. GUI Client** | 17-20 | Native GUI + emergent visualization |

**Total Duration**: **20 weeks (5 months)**  
**Key Innovation**: **Risk Management Emergente pipeline**  
**Expected Completion**: **March 2026**

---

## 🌟 **VALUE PROPOSITION ACTUALIZADO**

### **Competitive Advantages**:

#### ✅ **Scientific Risk Management**:
- First system to derive risk management from signal behavior
- No arbitrary stop losses or take profits
- Data-driven optimization with confidence scores

#### ✅ **Multi-Asset Scalability**:
- Same strategy framework works across all asset classes
- Automatic volatility adjustments
- Cross-asset performance validation

#### ✅ **Pure Signal Quality**:
- Evaluate strategy logic independently of risk management
- True signal quality assessment without noise
- Optimized risk management per strategy type

#### ✅ **Production Ready Pipeline**:
- Complete screening → optimization → validation → deployment
- Realistic backtesting with all execution costs
- Live trading ready with warmup planning

---

**🎯 Next Action**: Begin **Fase 1** - Complete `data/multi_timeframe.rs` implementation

**Prepared by**: DarwinX Development Team  
**Last Updated**: October 2025  
**Status**: ✅ **Ready for Development v2.1 - Emergent Risk Edition**
