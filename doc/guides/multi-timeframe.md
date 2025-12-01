# 🎯 Guía Multi-Timeframe - DarwinX

## Visión General

El sistema multi-timeframe de DarwinX permite crear estrategias que combinan indicadores de diferentes timeframes para análisis más sofisticados.

## Conceptos Fundamentales

### Timeframe Categories

En lugar de timeframes absolutos, DarwinX usa categorías semánticas:

```rust
pub enum TimeframeCategory {
    Current,  // Timeframe principal de la estrategia
    Medium,   // 3-5x el timeframe principal  
    High,     // 12-24x el timeframe principal
}
```

### Mapping Automático

| Principal | Current | Medium | High | Use Case |
|-----------|---------|--------|------|----------|
| **1m** | 1m | 5m | 1h | Scalping + Context |
| **5m** | 5m | 15m | 1h | Day trading |
| **15m** | 15m | 1h | 4h | Swing trading |
| **1h** | 1h | 4h | 1d | Position trading |
| **4h** | 4h | 1d | 1w | Long-term |
| **1d** | 1d | 1w | 1M | Investment |

## Lógica de Evaluación

**Principio fundamental**: Higher timeframes = vela cerrada anterior

```rust
match indicator.timeframe_category {
    Current => get_current_value(timestamp),      // Vela actual
    Medium | High => get_last_closed_value(timestamp), // Última vela cerrada
}
```

## Ejemplo Práctico

**Strategy Timeline (Primary = 5m)**:
```
14:00 ■■■■■■■■■■■■ 1h closed → EMA(200) = 42,150
14:05 ▓▓▓ 5m eval → RSI = 28.5, EMA_1h = 42,150 ✅ SIGNAL  
14:10 ▓▓▓ 5m eval → RSI = 31.2, EMA_1h = 42,150 ❌ No signal
14:15 ▓▓▓ 5m eval → RSI = 29.8, EMA_1h = 42,150 ✅ SIGNAL
15:00 ■■■■■■■■■■■■ 1h closed → EMA(200) = 42,200 (updated)
```

## Impacto en Módulos

### Módulos con Alto Impacto
- **Backtest Engine**: Requiere sincronización de múltiples timeframes
- **Data Module**: Necesita cargar y alinear múltiples series temporales

### Diseño Híbrido Recomendado

Usar traits genéricos desde el inicio:

```rust
pub trait DataProvider {
    fn get_candle(&self, tf: TimeFrame, idx: usize) -> Option<&Candle>;
    fn get_timeframes(&self) -> &[TimeFrame];
}

// Implementación single-TF para MVP
pub struct SingleTimeFrameProvider { ... }

// Implementación MTF después (sin breaking changes)
pub struct MultiTimeFrameProvider { ... }
```

## Referencias

- [ADR-001: Multi-Timeframe](../architecture/decisions/ADR-001-multi-timeframe.md)
- [Especificaciones Completas](../specifications/complete.md#multi-timeframe)

