# 📊 Guía de Uso del Módulo Data

## 📋 Resumen

El módulo `darwinx-data` proporciona funcionalidades para cargar datos históricos de trading desde diferentes formatos (CSV, Parquet) y gestionar contextos multi-timeframe para estrategias que combinan indicadores de diferentes timeframes.

## 🚀 Uso Básico

### Cargar Datos desde CSV

```rust
use darwinx_data::CsvLoader;
use darwinx_core::Candle;

// Cargar datos desde un archivo CSV
let candles: Vec<Candle> = CsvLoader::load("data/btcusdt_5m.csv")?;

println!("Cargadas {} velas", candles.len());
```

**Formato CSV esperado**:
```csv
timestamp,open,high,low,close,volume
1609459200000,29000.0,29500.0,28800.0,29200.0,1500.5
1609462800000,29200.0,29800.0,29000.0,29500.0,1800.2
```

### Cargar Datos desde Parquet

```rust
use darwinx_data::ParquetLoader;

// Cargar datos desde un archivo Parquet
let candles: Vec<Candle> = ParquetLoader::load("data/btcusdt_5m.parquet")?;

println!("Cargadas {} velas", candles.len());
```

**Formato Parquet esperado**: Mismas columnas que CSV (timestamp, open, high, low, close, volume)

## 🎯 Multi-Timeframe Context

### Cargar Múltiples Timeframes

El `MultiTimeframeContext` permite trabajar con datos de múltiples timeframes simultáneamente, esencial para estrategias multi-timeframe.

#### Ejemplo: Cargar desde CSV

```rust
use darwinx_data::{MultiTimeframeLoader, MultiTimeframeContext};
use darwinx_core::TimeFrame;
use std::collections::HashMap;

// Definir paths para cada timeframe
let mut paths = HashMap::new();
paths.insert(TimeFrame::M5, "data/btcusdt_5m.csv");
paths.insert(TimeFrame::M15, "data/btcusdt_15m.csv");
paths.insert(TimeFrame::H1, "data/btcusdt_1h.csv");

// Cargar todos los timeframes (M5 es el principal)
let context = MultiTimeframeLoader::load_multi_csv(&paths, TimeFrame::M5)?;

println!("Timeframes cargados: {:?}", context.available_timeframes());
```

#### Ejemplo: Cargar desde Parquet

```rust
let mut paths = HashMap::new();
paths.insert(TimeFrame::M5, "data/btcusdt_5m.parquet");
paths.insert(TimeFrame::H1, "data/btcusdt_1h.parquet");

let context = MultiTimeframeLoader::load_multi_parquet(&paths, TimeFrame::M5)?;
```

#### Ejemplo: Cargar un Solo Timeframe

```rust
// Cargar solo un timeframe (pero usando la API multi-timeframe)
let context = MultiTimeframeLoader::load_single_csv(
    "data/btcusdt_5m.csv",
    TimeFrame::M5
)?;
```

### Usar el Contexto Multi-Timeframe

```rust
use darwinx_data::MultiTimeframeContext;
use darwinx_core::TimeFrame;

// Obtener datos de un timeframe específico
if let Some(tf_data) = context.get_timeframe(TimeFrame::H1) {
    let closes = tf_data.close(20); // Últimas 20 velas cerradas
    println!("Últimos 20 closes en H1: {:?}", closes);
}

// Obtener el timeframe principal
let primary = context.primary_timeframe();
println!("Timeframe principal: {:?}", primary);

// Listar todos los timeframes disponibles
let available = context.available_timeframes();
println!("Timeframes disponibles: {:?}", available);
```

## 🔄 Sincronización de Timeframes

El módulo incluye un `TimeframeSynchronizer` para sincronizar datos de diferentes timeframes usando forward-fill.

```rust
use darwinx_data::{MultiTimeframeContext, TimeframeSynchronizer};
use darwinx_core::TimeFrame;

let context = MultiTimeframeLoader::load_multi_csv(&paths, TimeFrame::M5)?;
let synchronizer = TimeframeSynchronizer::new(&context);

// Obtener datos sincronizados para un timestamp específico
let timestamp = 1609459200000;
let synced_data = synchronizer.get_synced_data(timestamp)?;

// Acceder a datos de diferentes timeframes sincronizados
if let Some(h1_candle) = synced_data.get(&TimeFrame::H1) {
    println!("H1 candle en timestamp {}: {:?}", timestamp, h1_candle);
}
```

## 💾 Cache de Datos

El `MultiTimeframeDataCache` proporciona caching eficiente de datos multi-timeframe:

```rust
use darwinx_data::MultiTimeframeDataCache;

let mut cache = MultiTimeframeDataCache::new();

// Agregar datos al cache
cache.add_timeframe(TimeFrame::M5, candles_5m);
cache.add_timeframe(TimeFrame::H1, candles_1h);

// Obtener datos del cache
if let Some(cached) = cache.get_timeframe(TimeFrame::H1) {
    // Usar datos cacheados
}
```

## 📐 Alineación Temporal

El `TimeframeAligner` ayuda a alinear datos de diferentes timeframes:

```rust
use darwinx_data::TimeframeAligner;

let aligner = TimeframeAligner::new();

// Alinear datos de H1 con timestamps de M5
let aligned = aligner.align_to_timeframe(
    &h1_candles,
    &m5_timestamps,
    TimeFrame::H1
)?;
```

## 📝 Ejemplo Completo

```rust
use darwinx_data::{MultiTimeframeLoader, MultiTimeframeContext};
use darwinx_core::TimeFrame;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    // 1. Cargar datos multi-timeframe
    let mut paths = HashMap::new();
    paths.insert(TimeFrame::M5, "data/btcusdt_5m.csv");
    paths.insert(TimeFrame::M15, "data/btcusdt_15m.csv");
    paths.insert(TimeFrame::H1, "data/btcusdt_1h.csv");
    
    let context = MultiTimeframeLoader::load_multi_csv(&paths, TimeFrame::M5)?;
    
    // 2. Acceder a datos de diferentes timeframes
    let m5_data = context.get_timeframe(TimeFrame::M5)
        .ok_or_else(|| anyhow::anyhow!("M5 data not found"))?;
    let h1_data = context.get_timeframe(TimeFrame::H1)
        .ok_or_else(|| anyhow::anyhow!("H1 data not found"))?;
    
    // 3. Obtener últimos valores
    let m5_closes = m5_data.close(20);
    let h1_closes = h1_data.close(20);
    
    println!("Últimos 20 closes M5: {:?}", m5_closes);
    println!("Últimos 20 closes H1: {:?}", h1_closes);
    
    // 4. Usar en estrategia multi-timeframe
    // Ejemplo: EMA 200 en H1, RSI 14 en M5
    // ...
    
    Ok(())
}
```

## 🔧 Integración con Backtest Engine

El módulo Data se integra directamente con el Backtest Engine:

```rust
use darwinx_data::MultiTimeframeLoader;
use darwinx_backtest_engine::{MultiTimeFrameProvider, BacktestEngine};

// Cargar datos
let context = MultiTimeframeLoader::load_multi_csv(&paths, TimeFrame::M5)?;

// Crear provider para backtest
let provider = MultiTimeFrameProvider::new(context);

// Ejecutar backtest
let engine = PolarsBacktestEngine::new();
let results = engine.run_backtest(&provider, &strategy)?;
```

## ⚠️ Notas Importantes

1. **Formato de Timestamps**: Los timestamps deben estar en milisegundos (Unix timestamp * 1000)
2. **Orden de Datos**: Los datos deben estar ordenados cronológicamente (más antiguo primero)
3. **Tipos de Datos**: Los precios y volúmenes deben ser `f64` (Float64)
4. **Headers CSV**: Los archivos CSV deben tener headers: `timestamp,open,high,low,close,volume`
5. **Sincronización**: El forward-fill se aplica automáticamente cuando se accede a timeframes superiores

## 🐛 Manejo de Errores

Todos los métodos retornan `Result` para manejo robusto de errores:

```rust
match CsvLoader::load("data/file.csv") {
    Ok(candles) => {
        println!("Cargadas {} velas", candles.len());
    }
    Err(e) => {
        eprintln!("Error al cargar datos: {}", e);
    }
}
```

## 📚 API Reference

### Loaders

- `CsvLoader::load(path: &str) -> Result<Vec<Candle>>`
- `ParquetLoader::load(path: &str) -> Result<Vec<Candle>>`
- `MultiTimeframeLoader::load_multi_csv(paths, primary) -> Result<MultiTimeframeContext>`
- `MultiTimeframeLoader::load_multi_parquet(paths, primary) -> Result<MultiTimeframeContext>`
- `MultiTimeframeLoader::load_single_csv(path, timeframe) -> Result<MultiTimeframeContext>`
- `MultiTimeframeLoader::load_single_parquet(path, timeframe) -> Result<MultiTimeframeContext>`

### MultiTimeframeContext

- `new(primary_timeframe: TimeFrame) -> Self`
- `add_timeframe(timeframe: TimeFrame, candles: Vec<Candle>)`
- `get_timeframe(timeframe: TimeFrame) -> Option<&TimeframeData>`
- `primary_timeframe() -> TimeFrame`
- `available_timeframes() -> Vec<TimeFrame>`

### TimeframeData

- `close(lookback: usize) -> Vec<f64>`
- `high(lookback: usize) -> Vec<f64>`
- `low(lookback: usize) -> Vec<f64>`
- `volume(lookback: usize) -> Vec<f64>`
- `candle_at_timestamp(timestamp: i64) -> Option<&Candle>`
- `len() -> usize`

