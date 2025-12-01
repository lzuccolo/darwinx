# 📊 Indicadores Disponibles para el Generador

## 🎯 Resumen

El generador de estrategias usa **todos los indicadores registrados** en el `IndicatorRegistry` de forma **100% dinámica**. Esto significa que:

- ✅ No hay lista hardcodeada de indicadores
- ✅ Cualquier indicador registrado puede ser usado
- ✅ Los parámetros se generan automáticamente según los rangos definidos en el metadata
- ✅ El sistema es extensible: agregar un nuevo indicador lo hace disponible automáticamente

## 📦 Indicadores Actualmente Disponibles

### 📈 Trend (Tendencia) - 4 indicadores

1. **SMA** (Simple Moving Average)
   - Parámetro: `period` (2.0 - 200.0, default: 20.0)
   - Archivo: `crates/indicators/src/trend/sma.rs`
   - ✅ Implementado en backtest Polars

2. **EMA** (Exponential Moving Average)
   - Parámetro: `period` (2.0 - 200.0, default: 12.0)
   - Archivo: `crates/indicators/src/trend/ema.rs`
   - ✅ Implementado en backtest Polars

3. **WMA** (Weighted Moving Average)
   - Parámetro: `period` (2.0 - 200.0, default: 20.0)
   - Archivo: `crates/indicators/src/trend/wma.rs`
   - ⚠️ Pendiente implementar en backtest Polars

4. **VWMA** (Volume Weighted Moving Average)
   - Parámetro: `period` (2.0 - 200.0, default: 20.0)
   - Archivo: `crates/indicators/src/trend/vwma.rs`
   - ⚠️ Pendiente implementar en backtest Polars

### 📊 Momentum (Momento) - 4 indicadores

1. **RSI** (Relative Strength Index)
   - Parámetro: `period` (2.0 - 100.0, default: 14.0)
   - Archivo: `crates/indicators/src/momentum/rsi.rs`
   - ✅ Implementado en backtest Polars

2. **MACD** (Moving Average Convergence Divergence)
   - Parámetros: `fast` (2.0 - 50.0, default: 12.0), `slow` (2.0 - 100.0, default: 26.0), `signal` (2.0 - 50.0, default: 9.0)
   - Archivo: `crates/indicators/src/momentum/macd.rs`
   - ⚠️ Pendiente implementar en backtest Polars

3. **Stochastic**
   - Parámetros: `k_period` (2.0 - 50.0, default: 14.0), `d_period` (2.0 - 50.0, default: 3.0)
   - Archivo: `crates/indicators/src/momentum/stochastic.rs`
   - ⚠️ Pendiente implementar en backtest Polars

4. **ROC** (Rate of Change)
   - Parámetro: `period` (2.0 - 100.0, default: 12.0)
   - Archivo: `crates/indicators/src/momentum/roc.rs`
   - ⚠️ Pendiente implementar en backtest Polars

### 📉 Volatility (Volatilidad) - 3 indicadores

1. **ATR** (Average True Range)
   - Parámetro: `period` (2.0 - 100.0, default: 14.0)
   - Archivo: `crates/indicators/src/volatility/atr.rs`
   - ⚠️ Pendiente implementar en backtest Polars

2. **Bollinger Bands**
   - Parámetros: `period` (2.0 - 200.0, default: 20.0), `std_dev` (0.5 - 5.0, default: 2.0)
   - Archivo: `crates/indicators/src/volatility/bollinger.rs`
   - ⚠️ Pendiente implementar en backtest Polars

3. **Keltner Channels**
   - Parámetros: `period` (2.0 - 200.0, default: 20.0), `multiplier` (0.5 - 5.0, default: 2.0)
   - Archivo: `crates/indicators/src/volatility/keltner.rs`
   - ⚠️ Pendiente implementar en backtest Polars

### 📊 Volume (Volumen) - 3 indicadores

1. **OBV** (On-Balance Volume)
   - Sin parámetros
   - Archivo: `crates/indicators/src/volume/obv.rs`
   - ⚠️ Pendiente implementar en backtest Polars

2. **MFI** (Money Flow Index)
   - Parámetro: `period` (2.0 - 100.0, default: 14.0)
   - Archivo: `crates/indicators/src/volume/mfi.rs`
   - ⚠️ Pendiente implementar en backtest Polars

3. **VWAP** (Volume Weighted Average Price)
   - Sin parámetros
   - Archivo: `crates/indicators/src/volume/vwap.rs`
   - ⚠️ Pendiente implementar en backtest Polars

## 📊 Total: 14 Indicadores Disponibles

| Categoría | Cantidad | Implementados en Polars |
|-----------|----------|------------------------|
| Trend | 4 | 2 (SMA, EMA) |
| Momentum | 4 | 1 (RSI) |
| Volatility | 3 | 0 |
| Volume | 3 | 0 |
| **Total** | **14** | **3** |

## 🔧 Cómo Funciona el Generador

### Selección Dinámica

```rust
// El generador obtiene TODOS los indicadores del registry
let available = registry::all_names();  // Retorna: ["sma", "ema", "rsi", "macd", ...]

// Selecciona uno aleatorio
let selected_name = available.choose(rng).unwrap();

// Obtiene metadata del indicador
let meta = registry::get(selected_name).unwrap();

// Genera parámetros aleatorios dentro de los rangos válidos
let params: Vec<f64> = meta.parameters
    .iter()
    .map(|param_def| rng.gen_range(param_def.min..=param_def.max))
    .collect();
```

### Ejemplo de Estrategia Generada

```rust
// El generador puede crear estrategias como:
StrategyAST {
    name: "Strategy_0",
    timeframe: TimeFrame::H1,
    entry_rules: RuleSet {
        operator: LogicalOperator::And,
        conditions: [
            Condition {
                indicator: IndicatorType { name: "rsi", params: [14.0] },
                comparison: Comparison::LessThan,
                value: ConditionValue::Number(30.0)
            },
            Condition {
                indicator: IndicatorType { name: "sma", params: [20.0] },
                comparison: Comparison::GreaterThan,
                value: ConditionValue::Price
            }
        ]
    },
    exit_rules: ...
}
```

## ✅ Estado de Implementación en Backtest Polars

### ✅ Todos los Indicadores Implementados (14/14)

#### Trend (4/4)
- **SMA**: Simple Moving Average ✅
- **EMA**: Exponential Moving Average ✅
- **WMA**: Weighted Moving Average ✅
- **VWMA**: Volume Weighted Moving Average ✅

#### Momentum (4/4)
- **RSI**: Relative Strength Index ✅
- **MACD**: Moving Average Convergence Divergence ✅ (usa macd_line)
- **Stochastic**: Stochastic Oscillator ✅
- **ROC**: Rate of Change ✅

#### Volatility (3/3)
- **ATR**: Average True Range ✅
- **Bollinger Bands**: Bollinger Bands ✅ (usa middle band)
- **Keltner Channels**: Keltner Channels ✅ (usa middle channel)

#### Volume (3/3)
- **OBV**: On-Balance Volume ✅
- **MFI**: Money Flow Index ✅
- **VWAP**: Volume Weighted Average Price ✅

**Nota**: Los indicadores que retornan múltiples valores (MACD, Bollinger, Keltner) usan el valor principal (macd_line, middle band/channel) para las comparaciones en las estrategias.

## 🚀 Extensibilidad

Para agregar un nuevo indicador:

1. Crear archivo en la categoría correspondiente (ej: `crates/indicators/src/trend/new_indicator.rs`)
2. Implementar la función del indicador
3. Crear función `metadata()` con `register_indicator!(metadata)`
4. **¡Listo!** El generador lo usará automáticamente

## 📝 Notas

- El generador puede usar **cualquier combinación** de estos 14 indicadores
- Los parámetros se generan aleatoriamente dentro de los rangos válidos
- El backtest Polars actualmente solo soporta 3 indicadores (SMA, EMA, RSI)
- Los demás indicadores usarán `col("close")` como fallback temporal hasta ser implementados

