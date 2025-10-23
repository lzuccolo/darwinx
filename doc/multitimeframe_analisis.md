# 🎯 Multi-Timeframe en Strategy Generator - Análisis y Propuesta

## 📊 Estado Actual

### ❌ Actualmente NO Soporta Multi-Timeframe

Tu `StrategyAST` actual solo tiene:

```rust
pub struct StrategyAST {
    pub name: String,
    pub timeframe: TimeFrame,  // ⚠️ Solo UN timeframe
    pub entry_rules: RuleSet,
    pub exit_rules: RuleSet,
}
```

### ✅ Las Especificaciones SÍ Requieren Multi-Timeframe

Del archivo `especificaciones_arquitectura.md`:

```json
{
  "name": "MA_Cross_RSI_Filter",
  "timeframes": {
    "primary": "1h",           // ⭐ Timeframe principal
    "secondary": ["4h", "1d"]  // ⭐ Timeframes secundarios
  },
  "entry_rules": {
    "conditions": [
      {
        "indicator": "rsi",
        "timeframe": "4h",  // ⭐ Condición usa timeframe específico
        "params": { "period": 14 },
        "comparison": "greater_than",
        "value": 50
      }
    ]
  }
}
```

**Conclusión:** Tu implementación actual está incompleta para el objetivo final del proyecto.

---

## 🎯 Objetivos del Proyecto (Recordatorio)

De las especificaciones:

> **"Soporte multi-timeframe para estrategias complejas"**

Ejemplo de estrategia MTF:
- **Timeframe principal (1h)**: Detectar cruces de medias móviles
- **Timeframe secundario (4h)**: Confirmar tendencia con RSI
- **Timeframe secundario (1d)**: Verificar contexto de mercado

---

## 🔧 Propuesta de Implementación

### Opción 1: Modificación Mínima (RECOMENDADO PARA AHORA)

**Mantener la estructura actual** y agregar multi-timeframe en Fase 4 cuando implementes el backtest engine.

**Razón:** 
- El generador actual funciona perfecto para estrategias single-timeframe
- Multi-timeframe agrega complejidad significativa
- Es mejor tener algo funcionando simple que algo complejo a medias
- Puedes generar miles de estrategias single-TF ahora mismo

**Acción:** ✅ **Continuar a Fase 4** con estrategias single-timeframe

---

### Opción 2: Implementación Completa de Multi-Timeframe (FUTURO)

Cuando estés listo (después de Fase 4), implementar:

#### 1. Modificar `StrategyAST`

```rust
// crates/strategy-generator/src/ast/nodes.rs

pub struct StrategyAST {
    pub name: String,
    pub primary_timeframe: TimeFrame,      // Timeframe principal
    pub secondary_timeframes: Vec<TimeFrame>, // Timeframes adicionales
    pub entry_rules: RuleSet,
    pub exit_rules: RuleSet,
}

impl StrategyAST {
    pub fn new(name: String, primary_tf: TimeFrame) -> Self {
        Self {
            name,
            primary_timeframe: primary_tf,
            secondary_timeframes: Vec::new(), // Por defecto vacío
            entry_rules: RuleSet::default(),
            exit_rules: RuleSet::default(),
        }
    }
    
    /// Agrega un timeframe secundario
    pub fn add_secondary_timeframe(&mut self, tf: TimeFrame) {
        if !self.secondary_timeframes.contains(&tf) && tf != self.primary_timeframe {
            self.secondary_timeframes.push(tf);
        }
    }
    
    /// Verifica si la estrategia usa múltiples timeframes
    pub fn is_multi_timeframe(&self) -> bool {
        !self.secondary_timeframes.is_empty()
    }
    
    /// Retorna todos los timeframes usados
    pub fn all_timeframes(&self) -> Vec<TimeFrame> {
        let mut tfs = vec![self.primary_timeframe];
        tfs.extend(self.secondary_timeframes.iter().copied());
        tfs
    }
}
```

#### 2. Modificar `Condition` para Soportar Timeframe Específico

```rust
pub struct Condition {
    pub indicator: IndicatorType,
    pub comparison: Comparison,
    pub value: ConditionValue,
    pub timeframe: Option<TimeFrame>, // ⭐ NUEVO: timeframe específico para esta condición
}

impl Condition {
    /// Obtiene el timeframe de esta condición (o None si usa el principal)
    pub fn timeframe(&self) -> Option<TimeFrame> {
        self.timeframe
    }
    
    /// Establece un timeframe específico para esta condición
    pub fn with_timeframe(mut self, tf: TimeFrame) -> Self {
        self.timeframe = Some(tf);
        self
    }
}
```

#### 3. Actualizar `RandomGenerator`

```rust
// crates/strategy-generator/src/generator/random.rs

impl RandomGenerator {
    /// Genera una estrategia, opcionalmente multi-timeframe
    pub fn generate_with_mtf(&self, use_mtf: bool) -> StrategyAST {
        let primary_tf = self.random_timeframe();
        let mut strategy = StrategyAST::new(self.generate_name(), primary_tf);
        
        // Agregar timeframes secundarios si multi-timeframe
        if use_mtf {
            let num_secondary = self.rng.borrow_mut().gen_range(1..=2);
            for _ in 0..num_secondary {
                let secondary_tf = self.random_secondary_timeframe(primary_tf);
                strategy.add_secondary_timeframe(secondary_tf);
            }
        }
        
        // Generar condiciones (algunas pueden usar timeframes secundarios)
        let num_conditions = self.rng.borrow_mut().gen_range(2..=5);
        for _ in 0..num_conditions {
            let mut condition = self.random_condition();
            
            // 30% probabilidad de usar timeframe secundario si disponible
            if use_mtf && !strategy.secondary_timeframes.is_empty() 
               && self.rng.borrow_mut().gen_bool(0.3) 
            {
                let tf = strategy.secondary_timeframes
                    .choose(&mut *self.rng.borrow_mut())
                    .copied()
                    .unwrap();
                condition.timeframe = Some(tf);
            }
            
            strategy.entry_rules.conditions.push(condition);
        }
        
        strategy
    }
    
    /// Genera un timeframe secundario diferente al primario
    fn random_secondary_timeframe(&self, primary: TimeFrame) -> TimeFrame {
        let all_tfs = [
            TimeFrame::M1, TimeFrame::M5, TimeFrame::M15, TimeFrame::M30,
            TimeFrame::H1, TimeFrame::H4, TimeFrame::D1, TimeFrame::W1,
        ];
        
        let mut rng = self.rng.borrow_mut();
        loop {
            let tf = *all_tfs.choose(&mut *rng).unwrap();
            if tf != primary {
                return tf;
            }
        }
    }
}
```

#### 4. Actualizar `GeneticGenerator`

```rust
// crates/strategy-generator/src/generator/genetic.rs

impl GeneticGenerator {
    /// Genera población con probabilidad de multi-timeframe
    pub fn generate_population_mtf(&self, count: usize, mtf_probability: f64) -> Vec<StrategyAST> {
        (0..count)
            .map(|_| {
                let use_mtf = rand::thread_rng().gen_bool(mtf_probability);
                self.random_gen.generate_with_mtf(use_mtf)
            })
            .collect()
    }
}
```

#### 5. Actualizar Serialización JSON

```rust
// En StrategyAST
impl StrategyAST {
    pub fn to_json_mtf(&self) -> serde_json::Value {
        json!({
            "name": self.name,
            "timeframes": {
                "primary": format!("{:?}", self.primary_timeframe),
                "secondary": self.secondary_timeframes.iter()
                    .map(|tf| format!("{:?}", tf))
                    .collect::<Vec<_>>()
            },
            "entry_rules": {
                "operator": format!("{:?}", self.entry_rules.operator),
                "conditions": self.entry_rules.conditions.iter()
                    .map(|c| {
                        let mut obj = json!({
                            "indicator": c.indicator.name(),
                            "comparison": format!("{:?}", c.comparison),
                            "value": match &c.value {
                                ConditionValue::Number(n) => json!(n),
                                ConditionValue::Price => json!("price"),
                                ConditionValue::Indicator(ind) => json!(ind.name()),
                            }
                        });
                        
                        // Agregar timeframe si está especificado
                        if let Some(tf) = c.timeframe {
                            obj["timeframe"] = json!(format!("{:?}", tf));
                        }
                        
                        obj
                    })
                    .collect::<Vec<_>>()
            },
            "exit_rules": {
                // Similar...
            }
        })
    }
}
```

---

## 📋 Plan de Implementación Completo

### Fase 3.5: Multi-Timeframe Support (OPCIONAL - 2 semanas)

| # | Tarea | Tiempo | Prioridad |
|---|-------|--------|-----------|
| 1 | Modificar `StrategyAST` con campos MTF | 2h | Alta |
| 2 | Agregar `timeframe` a `Condition` | 1h | Alta |
| 3 | Actualizar `RandomGenerator` para MTF | 4h | Alta |
| 4 | Actualizar `GeneticGenerator` para MTF | 3h | Alta |
| 5 | Actualizar serialización JSON | 2h | Media |
| 6 | Tests de estrategias MTF | 4h | Alta |
| 7 | Validator para MTF | 3h | Media |
| 8 | Documentación y ejemplos | 3h | Media |
| 9 | Constraints para MTF | 2h | Media |

**Total:** ~24 horas (3 días)

---

## 🎯 Recomendación Final

### Para AHORA (MVP):

✅ **Opción 1: Mantener Single-Timeframe**

**Razones:**
1. El generador actual funciona perfectamente
2. Puedes avanzar a Fase 4 (Backtest Engine) sin bloqueos
3. Multi-timeframe es un "nice to have", no un bloqueante
4. Es mejor tener MVP funcionando que feature incompleto

**Acción:**
```bash
# Marcar en el roadmap
echo "- [ ] Fase 3.5: Multi-Timeframe Support (POSTPONED to post-MVP)" >> roadmap.md

# Continuar con Fase 4
echo "✅ Comenzar Fase 4: Backtest Engine"
```

### Para DESPUÉS (Post-MVP):

✅ **Implementar Multi-Timeframe** como mejora

**Cuándo:**
- Después de tener Fase 4-8 funcionando (MVP completo)
- Cuando tengas backtest engine funcionando
- Como feature de v0.4.0 o v0.5.0

---

## 📊 Comparación de Estrategias

### Single-Timeframe (Actual)

```rust
// Estrategia actual - FUNCIONA
StrategyAST {
    name: "SMA_Cross",
    timeframe: TimeFrame::H1,
    entry: [
        RSI(14) > 50,
        SMA(10) crosses_above SMA(30)
    ]
}
```

**Ventajas:**
- ✅ Simple de implementar
- ✅ Simple de testear
- ✅ Ya funciona
- ✅ Cubre 80% de casos de uso

### Multi-Timeframe (Futuro)

```rust
// Estrategia MTF - PENDIENTE
StrategyAST {
    name: "MTF_Trend_Filter",
    primary_timeframe: TimeFrame::H1,
    secondary_timeframes: [TimeFrame::H4, TimeFrame::D1],
    entry: [
        RSI(14) @ H1 > 50,           // Timeframe principal
        SMA(10) @ H4 crosses_above,  // Timeframe secundario
        EMA(200) @ D1 > price        // Contexto macro
    ]
}
```

**Ventajas:**
- ✅ Estrategias más sofisticadas
- ✅ Mejor filtrado de señales
- ✅ Alineación con specs originales

**Desventajas:**
- ⚠️ Más complejo de implementar
- ⚠️ Más complejo de testear
- ⚠️ Requiere data module completo (Fase 2)
- ⚠️ Backtest más lento

---

## 💡 Respuesta Directa

**Tu Pregunta:** "¿El módulo strategy_generator va a generar estrategias multitimeframe?"

**Respuesta Corta:** 
- **Actualmente:** NO ❌
- **Debería (según specs):** SÍ ✅
- **Recomendación:** Implementar POST-MVP (Fase 3.5 u 8.5)

**Respuesta Larga:**

1. Tu código actual genera solo single-timeframe
2. Las especificaciones originales requieren multi-timeframe
3. Es mejor terminar MVP primero con single-TF
4. Agregar MTF después es relativamente simple (~3 días)
5. MTF no es bloqueante para funcionalidad básica

---

## 🚀 Próximos Pasos Sugeridos

### Opción A: Continuar sin MTF (RECOMENDADO)

```bash
cd ~/shared/trading/src/darwinx

# Documentar la decisión
echo "## Decisión: Single-Timeframe First

- Estrategias single-TF para MVP
- Multi-timeframe se implementará en Fase 3.5 (post-MVP)
- Razón: Simplicidad y velocidad de desarrollo
" >> docs/DECISIONS.md

# Commit Fase 3
git add .
git commit -m "✅ Fase 3 completa - Single-timeframe strategies

Multi-timeframe postponed to post-MVP (Fase 3.5)"

# Comenzar Fase 4
mkdir -p crates/backtest-engine/src
```

### Opción B: Implementar MTF Ahora (3 días)

```bash
cd ~/shared/trading/src/darwinx

# Crear branch para feature
git checkout -b feature/multi-timeframe

# Implementar cambios (seguir plan de 24 horas arriba)

# Merge cuando esté listo
git checkout main
git merge feature/multi-timeframe
```

---

## 📝 Actualización del Roadmap

Agregar a `/mnt/project/roadmap.md`:

```markdown
## 🔮 FASE 3.5: Multi-Timeframe Support (Post-MVP)

**Duración:** 3 días  
**Estado:** POSTPONED  
**Dependencias:** Fase 2 completa, Fase 4 completa

### Objetivos
- [ ] Modificar StrategyAST para MTF
- [ ] Agregar timeframe a Condition
- [ ] Actualizar RandomGenerator
- [ ] Actualizar GeneticGenerator
- [ ] Tests MTF
- [ ] Documentación

### Justificación del Postponement
- MVP funciona con single-TF
- MTF agrega complejidad no bloqueante
- Mejor iterar sobre MVP completo primero
```

---

**Conclusión:** Tu generador actual NO soporta multi-timeframe, pero está PERFECTO para el MVP. Implementa MTF después de tener el sistema básico funcionando. 🎯