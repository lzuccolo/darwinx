# ✅ TODO - DarwinX

## 🔥 URGENTE (Esta Semana)

### Data Module - Completar Multi-Timeframe
- [x] ✅ MultiTimeframeContext - IMPLEMENTADO
- [x] ✅ TimeframeSynchronizer - IMPLEMENTADO  
- [x] ✅ MultiTimeframeDataCache - IMPLEMENTADO
- [x] ✅ TimeframeAligner - IMPLEMENTADO
- [ ] Integración con loaders (CSV/Parquet)
- [ ] Tests de integración end-to-end
- [ ] Documentación de uso

**Estado**: 95% completo - Solo falta integración

## 🚀 CRÍTICO (Próximas 2 Semanas)

### Backtest Engine
- [ ] Crear crate `crates/backtest-engine/`
- [ ] Implementar trait `DataProvider` (MTF-ready)
- [ ] Implementar `SingleTimeFrameProvider`
- [ ] Implementar `BacktestEngine<D: DataProvider>`
- [ ] Motor Polars vectorizado
- [ ] Métricas de performance (Sharpe, Sortino, etc.)
- [ ] Execution engine básico
- [ ] Tests comprehensivos

**Prioridad**: 🔥 CRÍTICA  
**Estimación**: 2 semanas  
**Bloquea**: Optimizer, Runner Live

### Strategy Converter Hub
- [ ] Crear crate `crates/strategy-converter/`
- [ ] Parser Rhai → AST
- [ ] AST → Rhai conversion
- [ ] AST → Rust conversion
- [ ] AST → Python conversion
- [ ] AST → Freqtrade conversion
- [ ] Tests de conversión bidireccional

**Prioridad**: 🔥 ALTA  
**Estimación**: 1-2 semanas  
**Bloquea**: GUI Client (editor Rhai)

## 📡 ALTA PRIORIDAD (Semana 3-4)

### API Server
- [ ] Crear crate `crates/api-server/`
- [ ] Implementar Strategy Service
- [ ] Implementar Backtest Service
- [ ] Implementar Optimizer Service
- [ ] Implementar Live Service
- [ ] Implementar Data Service
- [ ] Autenticación y autorización
- [ ] Tests de integración

**Prioridad**: 🔥 ALTA  
**Estimación**: 2 semanas

### API Client
- [ ] Crear crate `crates/api-client/`
- [ ] Cliente gRPC wrapper
- [ ] Reintentos y manejo de errores
- [ ] Tests

**Prioridad**: 🔥 MEDIA  
**Estimación**: 1 semana

## 💻 MEDIA PRIORIDAD (Semana 5-6)

### CLI Client
- [ ] Crear crate `crates/cli-client/`
- [ ] Comandos: generate, backtest, optimize, run
- [ ] Output formateado (tablas, JSON)
- [ ] Progress bars
- [ ] Tests

**Prioridad**: 🟡 MEDIA  
**Estimación**: 1-2 semanas

### Data Manager
- [ ] Crear crate `crates/data-manager/`
- [ ] Descarga de datos históricos
- [ ] Gestión de cache
- [ ] Warmup automático para indicadores
- [ ] Tests

**Prioridad**: 🟡 MEDIA  
**Estimación**: 1 semana

## 🎨 BAJA PRIORIDAD (Semana 7+)

### GUI Client
- [ ] Crear crate `crates/gui-client/`
- [ ] UI con GTK4/Relm4
- [ ] Editor Rhai integrado
- [ ] Visualización de resultados
- [ ] Dashboard de estrategias
- [ ] Tests

**Prioridad**: 🟢 BAJA  
**Estimación**: 3-4 semanas

### Optimizer
- [ ] Crear crate `crates/optimizer/`
- [ ] Grid search
- [ ] Genetic algorithm para optimización
- [ ] Walk-forward optimization
- [ ] Tests

**Prioridad**: 🟢 BAJA  
**Estimación**: 2 semanas  
**Depende**: Backtest Engine

### Runner Live
- [ ] Crear crate `crates/runner-live/`
- [ ] Integración con exchanges
- [ ] Order management
- [ ] Risk management
- [ ] Monitoring
- [ ] Tests

**Prioridad**: 🟢 BAJA  
**Estimación**: 3 semanas  
**Depende**: Backtest Engine, API Server

## 🔧 MEJORAS Y REFACTORING

### Strategy Generator
- [ ] Implementar correlation matrix real (Phase 3)
- [ ] Semantic constraints completos
- [ ] Genetic algorithm mejorado (Phase 4)
- [ ] Performance optimization

### Core
- [ ] Agregar más tipos de órdenes (Stop Loss, Take Profit)
- [ ] Mejorar error handling
- [ ] Documentación de traits

### Indicators
- [ ] Agregar más indicadores (ADX, CCI, etc.)
- [ ] Optimización de performance
- [ ] Tests de precisión

### Strategy Store
- [ ] Agregar índices para queries frecuentes
- [ ] Migración de datos
- [ ] Backup/restore

## 📝 DOCUMENTACIÓN

- [x] Reorganizar documentación ✅
- [ ] Completar guías de usuario
- [ ] Agregar ejemplos de código
- [ ] Documentar APIs públicas
- [ ] Crear tutoriales paso a paso

## 🧪 TESTING

- [ ] Aumentar coverage de tests
- [ ] Tests de integración end-to-end
- [ ] Tests de performance
- [ ] Tests de carga
- [ ] Property-based testing

## 🐛 BUGS CONOCIDOS

- [ ] Revisar y documentar bugs conocidos
- [ ] Priorizar fixes

## 📊 MÉTRICAS Y MONITOREO

- [ ] Implementar logging estructurado
- [ ] Métricas de performance
- [ ] Health checks
- [ ] Alertas

