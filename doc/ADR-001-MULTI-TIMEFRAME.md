# 🎯 Decisión Arquitectónica: Multi-Timeframe Strategy

## 📋 Contexto

Durante el desarrollo de la Fase 3 (Strategy Generator), surgió la pregunta:
> "¿El módulo strategy_generator va a generar estrategias multitimeframe?"

Esto llevó a un análisis del impacto en módulos futuros.

---

## 🔍 Hallazgos

### Estado Actual
- ✅ Strategy Generator: Funcional pero **solo single-timeframe**
- ✅ 34/34 tests pasando
- ✅ Listo para comenzar Fase 4

### Requerimientos Originales
- 📋 Especificaciones requieren **multi-timeframe**
- 📋 Ejemplo en specs muestra estrategias con TF primario + secundarios

### Impacto si se Agrega MTF Después

| Módulo | Impacto | Esfuerzo Refactor |
|--------|---------|-------------------|
| Backtest Engine | 🔴 CRÍTICO | 2-3 semanas |
| Data Module | 🔴 CRÍTICO | 1-2 semanas |
| Converter | 🟡 MEDIO | 1 semana |
| Optimizer | 🟡 MEDIO | 3-5 días |
| Live Runner | 🟡 MEDIO | 1 semana |
| Otros | 🟢 BAJO | 1-2 días |

**Total refactor post-MVP:** 4-6 semanas

---

## 🎯 Opciones Evaluadas

### Opción A: Single-Timeframe Puro
- MVP: 16 semanas
- Refactor MTF: +6 semanas
- **Total: 22 semanas**
- Riesgo: Alto (breaking changes)

### Opción B: Multi-Timeframe Completo Ahora
- MVP: 17 semanas
- Refactor MTF: 0 semanas
- **Total: 17 semanas**
- Riesgo: Bajo

### Opción C: Diseño Híbrido (Interfaces MTF-Ready) ⭐
- MVP: 16.5 semanas
- Refactor MTF: +2 semanas
- **Total: 18.5 semanas**
- Riesgo: Muy bajo

---

## ✅ Decisión: Opción C - Diseño Híbrido

### Justificación

1. **Balance óptimo** entre velocidad y flexibilidad
2. **Bajo riesgo** de breaking changes futuros
3. **Arquitectura correcta** desde el principio
4. **Ahorro significativo** vs Opción A (4 semanas menos)
5. **Overhead mínimo** vs single-TF puro (+3-4 días)

### Principios del Diseño Híbrido

```rust
// ✅ CORRECTO: Diseñar con abstracción
pub trait DataProvider {
    fn get_candle(&self, tf: TimeFrame, idx: usize) -> Option<&Candle>;
    fn get_timeframes(&self) -> &[TimeFrame];
}

// ✅ Implementación single-TF para MVP
pub struct SingleTFProvider { ... }

// ✅ Implementación MTF para después (solo agregar)
pub struct MultiTFProvider { ... }

// ❌ EVITAR: Asumir single-TF en toda la base de código
pub struct BacktestEngine {
    data: DataFrame,  // ❌ Difícil de extender
}
```

---

## 📋 Plan de Implementación

### Fase 4: Backtest Engine (Semanas 8-9)

#### Semana 8: Diseño MTF-Ready

**Día 1-2: Definir Traits**
```rust
// crates/backtest-engine/src/traits.rs

/// Proveedor de datos que puede soportar single o multi-timeframe
pub trait DataProvider: Send + Sync {
    /// Obtiene una vela específica
    fn get_candle(&self, tf: TimeFrame, idx: usize) -> Option<&Candle>;
    
    /// Retorna todos los timeframes disponibles
    fn get_timeframes(&self) -> &[TimeFrame];
    
    /// Obtiene el timeframe primario
    fn primary_timeframe(&self) -> TimeFrame;
    
    /// Número total de velas en el timeframe primario
    fn len(&self) -> usize;
}

/// Evaluador de estrategia que puede trabajar con contexto multi-TF
pub trait StrategyEvaluator: Send + Sync {
    /// Evalúa la estrategia con el contexto actual
    fn evaluate<D: DataProvider>(&self, provider: &D, idx: usize) -> Signal;
}
```

**Día 3-5: Implementación Single-TF**
```rust
// crates/backtest-engine/src/providers/single_tf.rs

pub struct SingleTimeFrameProvider {
    timeframe: TimeFrame,
    data: DataFrame,
}

impl DataProvider for SingleTimeFrameProvider {
    fn get_candle(&self, tf: TimeFrame, idx: usize) -> Option<&Candle> {
        if tf != self.timeframe {
            return None;
        }
        // TODO: obtener candle del DataFrame
    }
    
    fn get_timeframes(&self) -> &[TimeFrame] {
        std::slice::from_ref(&self.timeframe)
    }
    
    fn primary_timeframe(&self) -> TimeFrame {
        self.timeframe
    }
    
    fn len(&self) -> usize {
        self.data.height()
    }
}
```

**Día 6-7: BacktestEngine Genérico**
```rust
// crates/backtest-engine/src/engine.rs

pub struct BacktestEngine<D: DataProvider> {
    data_provider: D,
    initial_capital: f64,
    commission: f64,
}

impl<D: DataProvider> BacktestEngine<D> {
    pub fn run<E: StrategyEvaluator>(
        &self,
        strategy: &E,
    ) -> BacktestResult {
        let mut equity = self.initial_capital;
        let mut position: Option<Position> = None;
        
        // Loop sobre el timeframe primario
        for idx in 0..self.data_provider.len() {
            let signal = strategy.evaluate(&self.data_provider, idx);
            
            match signal {
                Signal::Buy => { /* ... */ }
                Signal::Sell => { /* ... */ }
                _ => {}
            }
        }
        
        BacktestResult { /* ... */ }
    }
}
```

#### Semana 9: Tests y Documentación

**Tests Parametrizados:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_provider() -> impl DataProvider {
        SingleTimeFrameProvider::new(/* ... */)
    }
    
    #[test]
    fn test_backtest_with_provider() {
        let provider = create_test_provider();
        let engine = BacktestEngine::new(provider, 10000.0, 0.001);
        // Tests funcionan con cualquier DataProvider!
    }
}
```

---

### Post-MVP: Agregar Multi-Timeframe (2 semanas)

#### Semana 1: Implementación MTF

```rust
// crates/backtest-engine/src/providers/multi_tf.rs

pub struct MultiTimeFrameProvider {
    primary: TimeFrame,
    data: HashMap<TimeFrame, DataFrame>,
    synchronizer: TimeFrameSynchronizer,
}

impl DataProvider for MultiTimeFrameProvider {
    fn get_candle(&self, tf: TimeFrame, idx: usize) -> Option<&Candle> {
        // Sincronizar índice entre timeframes
        let aligned_idx = self.synchronizer.align_index(
            self.primary,
            idx,
            tf
        )?;
        
        self.data.get(&tf)?.get_candle(aligned_idx)
    }
    
    fn get_timeframes(&self) -> &[TimeFrame] {
        // Retornar todos los TFs disponibles
        self.data.keys().collect()
    }
    
    fn primary_timeframe(&self) -> TimeFrame {
        self.primary
    }
    
    fn len(&self) -> usize {
        self.data[&self.primary].height()
    }
}
```

#### Semana 2: Actualizar Strategy Generator

```rust
// Modificar StrategyAST para MTF
pub struct StrategyAST {
    pub primary_timeframe: TimeFrame,
    pub secondary_timeframes: Vec<TimeFrame>,
    // ...
}

// Condiciones con TF específico
pub struct Condition {
    pub indicator: IndicatorType,
    pub timeframe: Option<TimeFrame>,  // None = usa primario
    // ...
}
```

---

## 📊 Comparación de Esfuerzo

### Opción A: Single-TF → MTF (Sin Diseño Híbrido)

```
Semana 1-2:   Backtest Engine (single-TF)
Semana 3-4:   Tests y optimización
              [MVP Completo]
Semana 5-10:  REFACTOR MASIVO
              - Rediseñar BacktestEngine
              - Modificar Data Module
              - Actualizar Converter
              - Rehacer tests
              - Corregir bugs de migración
```

### Opción C: Diseño Híbrido (ELEGIDA)

```
Semana 1:     Diseño de traits + Single-TF impl
Semana 2:     Tests y documentación
              [MVP Completo - interfaces listas]
Semana 3:     Implementar MultiTFProvider
Semana 4:     Actualizar Strategy Generator
              [MTF Completo - NO breaking changes]
```

**Ahorro:** 4 semanas

---

## ✅ Checklist de Implementación

### Fase 4 (Ahora)

#### Backtest Engine
- [ ] Definir trait `DataProvider`
- [ ] Definir trait `StrategyEvaluator`
- [ ] Implementar `SingleTimeFrameProvider`
- [ ] `BacktestEngine<D: DataProvider>`
- [ ] Tests parametrizados
- [ ] Documentar asunciones single-TF

#### Data Module
- [ ] Estructura que soporte HashMap<TF, DF>
- [ ] API: `load()` y `load_multi()`
- [ ] Cache con key = (symbol, timeframe)

#### Strategy Store
- [ ] Campo `timeframes: Vec<String>` en schema
- [ ] Default: `["primary"]`
- [ ] Listo para agregar secundarios

---

### Post-MVP

#### Multi-TimeFrame Implementation
- [ ] Implementar `TimeFrameSynchronizer`
- [ ] Implementar `MultiTimeFrameProvider`
- [ ] Actualizar `StrategyAST` (agregar campos MTF)
- [ ] Actualizar `Condition` (agregar campo `timeframe`)
- [ ] Tests de estrategias MTF
- [ ] Ejemplo MTF completo

---

## 📝 Ejemplo de Uso Futuro

### MVP (Single-TF)
```rust
// Funciona con single-TF
let provider = SingleTimeFrameProvider::new(data, TimeFrame::H1);
let engine = BacktestEngine::new(provider, 10000.0, 0.001);
let result = engine.run(&strategy);
```

### Post-MVP (Multi-TF)
```rust
// Mismo código funciona con multi-TF!
let mut data = HashMap::new();
data.insert(TimeFrame::H1, h1_data);
data.insert(TimeFrame::H4, h4_data);

let provider = MultiTimeFrameProvider::new(data, TimeFrame::H1);
let engine = BacktestEngine::new(provider, 10000.0, 0.001);
let result = engine.run(&strategy);  // ✅ Mismo API!
```

---

## 🎯 Beneficios del Diseño Híbrido

1. ✅ **No breaking changes** - API pública no cambia
2. ✅ **Tests reutilizables** - Funcionan con cualquier provider
3. ✅ **Fácil agregar features** - Solo implementar nuevo trait
4. ✅ **Mantenible** - Separación clara de responsabilidades
5. ✅ **Extensible** - Agregar otros tipos de providers (streaming, etc)

---

## 📊 Métricas de Éxito

### MVP (Fase 4)
- [ ] BacktestEngine compila y funciona
- [ ] Tests pasan con SingleTFProvider
- [ ] Performance objetivo alcanzado
- [ ] Documentación completa de traits

### Post-MVP (MTF)
- [ ] MultiTFProvider implementado
- [ ] Tests pasan con ambos providers
- [ ] No breaking changes en API pública
- [ ] Refactor completado en 2 semanas (no 6)

---

## 🚀 Próxima Acción Inmediata

**ANTES de comenzar código de Fase 4:**

1. ✅ Aprobar esta decisión arquitectónica
2. ✅ Commitear en repo como `docs/ADR-001-multi-timeframe.md`
3. ✅ Actualizar roadmap con diseño híbrido
4. ✅ Crear issues en GitHub para:
   - [ ] Fase 4: Implementar traits MTF-ready
   - [ ] Post-MVP: Implementar MultiTFProvider

---

## 📝 Registro de Decisión

**Fecha:** 22 Octubre 2025  
**Decisión:** Opción C - Diseño Híbrido  
**Estado:** ✅ Aprobada  
**Participantes:** Equipo de desarrollo  

**Razón principal:** Balance óptimo entre velocidad de MVP y arquitectura correcta.

**Alternativas consideradas:**
- Opción A: Rechazada (refactor masivo)
- Opción B: Rechazada (retrasa MVP innecesariamente)

**Consecuencias:**
- Positivas: Bajo riesgo, arquitectura extensible, ahorro de tiempo
- Negativas: Overhead mínimo en Fase 4 (+3-4 días)

**Próxima revisión:** Post-MVP cuando se implemente MTF

---

**Firma:** Arquitecto del Proyecto  
**Estado:** Ready for Implementation ✅
