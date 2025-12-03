# ✅ TODO - DarwinX

## 🔥 URGENTE (Esta Semana)

### Data Module - Completar Multi-Timeframe
- [x] ✅ MultiTimeframeContext - IMPLEMENTADO
- [x] ✅ TimeframeSynchronizer - IMPLEMENTADO  
- [x] ✅ MultiTimeframeDataCache - IMPLEMENTADO
- [x] ✅ TimeframeAligner - IMPLEMENTADO
- [x] ✅ Integración con loaders (CSV/Parquet) - COMPLETADO
- [x] ✅ Tests de integración end-to-end - COMPLETADO
- [x] ✅ Documentación de uso - COMPLETADO

**Estado**: 100% completo - Integración finalizada

## 🚀 CRÍTICO (Próximas 2 Semanas)

### Backtest Engine
- [x] ✅ Crear crate `crates/backtest-engine/` - COMPLETADO
- [x] ✅ Implementar trait `DataProvider` (MTF-ready) - COMPLETADO
- [x] ✅ Implementar `SingleTimeFrameProvider` - COMPLETADO
- [x] ✅ Implementar `MultiTimeFrameProvider` - COMPLETADO
- [x] ✅ Métricas de performance (Sharpe, Sortino, etc.) - COMPLETADO
- [x] ✅ Motor Polars vectorizado (implementación básica completada)
- [x] ✅ Execution engine básico (implementado en motor Polars)
- [x] ✅ Tests comprehensivos - COMPLETADO

**Estado**: 100% completo - Backtest Engine funcional

**Prioridad**: 🔥 CRÍTICA  
**Estimación**: 2 semanas  
**Bloquea**: Optimizer, Runner Live

### Strategy Converter Hub
- [x] ✅ Crear crate `crates/strategy-converter/` - COMPLETADO
- [x] ✅ Estructura básica (error, formats, converter) - COMPLETADO
- [x] ✅ Stubs para parsers y generadores - COMPLETADO
- [ ] Parser Rhai → AST (implementación completa)
- [ ] AST → Rhai conversion (implementación completa)
- [ ] AST → Rust conversion (implementación completa)
- [ ] AST → Python conversion (implementación completa)
- [ ] AST → Freqtrade conversion (implementación completa)
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

### Database Support
- [ ] Crear crate `crates/database/` o módulo en core
- [ ] Definir traits Repository para cada entidad (StrategyRepository, BacktestRepository, etc.)
- [ ] Implementar modelos de datos (Strategy, BacktestResult, Metrics, etc.)
- [ ] Definir schema de base de datos (estrategias, backtests, resultados, métricas)
- [ ] Implementar SQLiteRepository (usando `rusqlite` o `sqlx`)
  - [ ] StrategyRepository para SQLite
  - [ ] BacktestRepository para SQLite
  - [ ] MetricsRepository para SQLite
- [ ] Sistema de migraciones para SQLite (usar `sqlx migrate` o similar)
- [ ] Connection pooling para SQLite
- [ ] Factory/Builder pattern para crear repositorios según configuración
- [ ] Tests de integración con SQLite
- [ ] Documentación de schema y uso

**Prioridad**: 🔥 ALTA  
**Estimación**: 1-2 semanas  
**Nota**: Empezar con SQLite para desarrollo y testing, arquitectura preparada para PostgreSQL

### PostgreSQL Support (Futuro)
- [ ] Implementar PostgreSQLRepository (usando `sqlx` o `tokio-postgres`)
  - [ ] StrategyRepository para PostgreSQL
  - [ ] BacktestRepository para PostgreSQL
  - [ ] MetricsRepository para PostgreSQL
- [ ] Migrar schema de SQLite a PostgreSQL
- [ ] Actualizar connection pooling para PostgreSQL
- [ ] Tests de integración con PostgreSQL
- [ ] Scripts de migración de datos (SQLite → PostgreSQL)
- [ ] Documentación de deployment y configuración

**Prioridad**: 🟡 MEDIA  
**Estimación**: 1 semana  
**Depende**: Database Support (SQLite con Repository pattern)

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

## 🔥 CRÍTICO - Backtest Masivo con Polars

### Polars Vectorized Backtest Engine (REAL)
- [x] ✅ Implementar backtest vectorizado real con Polars - COMPLETADO
- [x] ✅ Convertir datos a DataFrame de Polars - COMPLETADO
- [x] ✅ Usar expresiones de Polars para señales vectorizadas - COMPLETADO
- [x] ✅ Pre-cálculo de indicadores (14/14 implementados) - COMPLETADO
- [x] ✅ Implementar todos los indicadores (SMA, EMA, WMA, VWMA, RSI, MACD, Stochastic, ROC, ATR, Bollinger, Keltner, OBV, MFI, VWAP) - COMPLETADO
- [ ] Procesar múltiples estrategias en batch paralelo
- [ ] Optimizar para 10,000-100,000 estrategias
- [ ] Tests de performance y throughput

**Prioridad**: 🔥 CRÍTICA  
**Estimación**: 1-2 semanas  
**Bloquea**: Pipeline de generación masiva

### Pipeline de Generación y Backtest Masivo
- [ ] Crear crate `crates/massive-backtest/` o módulo en backtest-engine
- [ ] Sistema de generación masiva (10K-100K estrategias)
- [ ] Backtest screening masivo con Polars vectorizado
- [ ] Sistema de ranking/selección (top 100)
- [ ] Backtest detallado con Event-Driven para top 100
- [ ] Reporte de resultados y métricas comparativas
- [ ] Tests end-to-end del pipeline completo

**Prioridad**: 🔥 CRÍTICA  
**Estimación**: 2 semanas  
**Depende**: Polars Vectorized Backtest Engine

### Sistema de Ranking y Selección
- [ ] Definir métricas de ranking (Sharpe, Sortino, Profit Factor, etc.)
- [ ] Implementar sistema de scoring compuesto
- [ ] Filtros de calidad (min trades, min win rate, etc.)
- [ ] Selección top N estrategias
- [ ] Tests de ranking

**Prioridad**: 🔥 ALTA  
**Estimación**: 1 semana  
**Depende**: Backtest masivo

## 🔄 RETROALIMENTACIÓN Y APRENDIZAJE

### Persistencia de Mejores Estrategias (SQLite)
- [x] ✅ Integrar guardado de top N estrategias en SQLite usando `strategy-store` - COMPLETADO
- [x] ✅ Crear tabla `best_strategies` o usar `strategies` existente con flag `is_best` - COMPLETADO
- [x] ✅ Guardar métricas completas de backtest junto con estrategia - COMPLETADO
- [x] ✅ Sistema de deduplicación (evitar guardar estrategias idénticas) - COMPLETADO
- [x] ✅ Timestamp y metadata de ejecución (dataset, fecha, configuración) - COMPLETADO
- [x] ✅ Función para cargar mejores estrategias históricas desde SQLite - COMPLETADO
- [ ] Tests de persistencia y carga desde SQLite

**Prioridad**: 🔥 CRÍTICA  
**Estado**: 95% completo - Funcional, faltan tests  
**Nota**: SQLite es la fuente de verdad principal. JSON solo para consulta rápida de ejecución actual.

### JSON como Resultado de Ejecución (Opcional)
- [ ] Mantener opción `--output` para guardar JSON de ejecución actual
- [ ] JSON solo para análisis rápido inmediato, no para persistencia
- [ ] Documentar que SQLite es la persistencia principal
- [ ] Considerar eliminar JSON por defecto o hacerlo opcional explícito

**Prioridad**: 🟡 BAJA  
**Estimación**: 0.5 semanas  
**Depende**: Persistencia de Mejores Estrategias (SQLite)

### Integración Genetic Generator con Backtest
- [x] ✅ Crear función de fitness que combine métricas de backtest - COMPLETADO
- [x] ✅ Cargar mejores estrategias desde SQLite como población inicial - COMPLETADO
- [x] ✅ Integrar `GeneticGenerator` con resultados de backtest masivo - COMPLETADO
- [x] ✅ Evolución iterativa: backtest → guardar en SQLite → cargar → evolve → backtest - COMPLETADO
- [x] ✅ Configuración de parámetros genéticos (mutation rate, elite size, etc.) - COMPLETADO
- [ ] Tests de evolución end-to-end con persistencia SQLite

**Prioridad**: 🔥 ALTA  
**Estado**: 95% completo - Funcional, faltan tests  
**Depende**: Persistencia de Mejores Estrategias (SQLite)

### Mejoras en Generación de Estrategias
- [ ] Detectar timeframe de datos y filtrar estrategias incompatibles antes del backtest
- [ ] Agregar soporte para valores históricos de indicadores (RSI[2], EMA[-1], etc.)
- [ ] Validación semántica de comparaciones (warnings para combinaciones poco comunes)
- [ ] Implementar agregación de velas para timeframes mayores (H4 desde H1, D1 desde H4)
- [ ] Función `is_timeframe_compatible()` para filtrar estrategias
- [ ] Tests de compatibilidad de timeframes

**Prioridad**: 🔥 MEDIA  
**Estimación**: 1 semana  
**Depende**: Pipeline de Generación y Backtest Masivo

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

