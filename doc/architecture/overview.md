# 🏗️ DarwinX - Arquitectura General

**Versión**: 2.0  
**Última Actualización**: Octubre 2025

## Visión General

DarwinX es un ecosistema modular de trading algorítmico en Rust que permite generar, testear y ejecutar miles de estrategias de trading **multi-timeframe** de forma automatizada, con arquitectura cliente-servidor basada en gRPC.

## Arquitectura de Alto Nivel

```
┌─────────────────────────────────────────────────────────────┐
│                    CAPA DE CLIENTE                          │
│  ┌──────────────┐              ┌──────────────┐            │
│  │ GUI Client   │              │ CLI Client   │            │
│  │ (GTK4/Relm4) │              │   (Clap)     │            │
│  └──────┬───────┘              └──────┬───────┘            │
│         │                              │                    │
│         └──────────────┬───────────────┘                    │
│                        │                                    │
└────────────────────────┼────────────────────────────────────┘
                         │
                         │ gRPC (Tonic)
                         │
┌────────────────────────▼────────────────────────────────────┐
│                    CAPA DE SERVIDOR                         │
│  ┌──────────────────────────────────────────────────────┐  │
│  │         API Server (gRPC Services)                   │  │
│  │  Strategy │ Backtest │ Optimizer │ Live │ Data      │  │
│  └──────────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────┐
│                  CAPA DE LÓGICA                            │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                │
│  │Generator │  │Converter │  │ Backtest │                │
│  │(Genetic) │  │  (Hub)   │  │ (Dual)   │                │
│  └──────────┘  └──────────┘  └──────────┘                │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                │
│  │Optimizer │  │Runner    │  │Data      │                │
│  │(Grid/GA) │  │Live      │  │Manager   │                │
│  └──────────┘  └──────────┘  └──────────┘                │
└────────────────────────┬────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────┐
│                    CAPA DE DATOS                            │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Strategy Store (PostgreSQL)                         │  │
│  │  - Estrategias (AST + Rhai)                          │  │
│  │  - Resultados de backtest                            │  │
│  │  - Similarity scores                                  │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Crates del Proyecto

### ✅ Completados (50%)

1. **`darwinx-core`** (100%): Types y traits fundamentales
2. **`darwinx-indicators`** (100%): Sistema de indicadores con registry auto-registrado
3. **`darwinx-data`** (95%): Loaders y multi-timeframe (casi completo)
4. **`darwinx-strategy-store`** (95%): Persistencia PostgreSQL
5. **`darwinx-strategy-generator`** (100%): Generación automática con soporte multi-TF
6. **`darwinx-api-proto`** (100%): Protocol Buffers gRPC

### ⏳ Pendientes (50%)

7. **`darwinx-backtest-engine`**: Motor de backtest dual (Polars + Event-driven)
8. **`darwinx-strategy-converter`**: Hub de conversión entre formatos
9. **`darwinx-optimizer`**: Optimización de parámetros
10. **`darwinx-runner-live`**: Ejecución en vivo
11. **`darwinx-data-manager`**: Gestión de datos históricos
12. **`darwinx-api-server`**: Servidor gRPC
13. **`darwinx-api-client`**: Cliente gRPC
14. **`darwinx-cli-client`**: CLI
15. **`darwinx-gui-client`**: GUI

## Características Clave

### 1. Multi-Timeframe First
- Soporte nativo para estrategias multi-timeframe
- Sincronización avanzada de timeframes
- Diseño híbrido MTF-ready desde el inicio

### 2. Generación Automática
- Algoritmos genéticos
- Random generation
- Semantic constraints (anti-correlación)

### 3. Backtest Dual
- **Polars Engine**: Vectorizado, 10,000+ estrategias/hora
- **Event-Driven Engine**: Realista, ~100 estrategias/hora

### 4. Risk Management Emergente
- Pure signal analysis primero
- Risk management derivado de señales
- Temporal validation (out-of-sample)

### 5. Strategy Converter Hub
- AST como formato intermedio
- Conversión bidireccional
- Soporte múltiples formatos (Rhai, Rust, Python, Freqtrade)

## Flujo de Datos

```
Data Loader (CSV/Parquet)
    ↓
Multi-Timeframe Synchronizer
    ↓
Strategy Generator / Loader
    ↓
Backtest Engine (Polars/Event-Driven)
    ↓
Results → Strategy Store (PostgreSQL)
    ↓
API Server (gRPC)
    ↓
Client (CLI/GUI)
```

## Tecnologías Principales

- **Rust 2024**: Lenguaje principal
- **Polars**: Procesamiento de datos vectorizado
- **Tonic**: gRPC framework
- **SQLx**: PostgreSQL
- **Rhai**: Scripting de estrategias
- **GTK4/Relm4**: GUI nativa
- **Tokio**: Runtime async

## Principios de Diseño

1. **Modularidad**: Cada crate es independiente
2. **Extensibilidad**: Fácil agregar nuevos indicadores/estrategias
3. **Type Safety**: Máxima seguridad de tipos
4. **Performance**: Optimizado para throughput masivo
5. **Testabilidad**: Tests comprehensivos

Ver [Detalles Técnicos](./technical.md) para información de implementación.

