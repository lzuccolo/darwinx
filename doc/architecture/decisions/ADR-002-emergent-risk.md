# 🧬 ADR-002: Risk Management Emergente

**Fecha**: Octubre 2025  
**Estado**: ✅ Aprobada  
**Versión**: 2.2

## Contexto

Durante el diseño del sistema de backtesting, surgió la pregunta:
> "¿Debemos imponer risk management fijo (ej: 2% stop loss) en el screening masivo de estrategias?"

## Problema

**Risk Management Fijo + Single Period**:
- Imponer 2% stop loss arbitrario en screening masivo
- Estrategias optimizadas para UN solo período (overfitting temporal)
- Asset classes diferentes requieren risk management diferente
- Pérdida de estrategias prometedoras por configuración incorrecta

## Decisión

**Risk Management Emergente + Out-of-Sample Temporal Validation**

### Pipeline Actualizado

```
1. Pure Signal Screening (10,000 strategies, SIN risk management)
   ↓
2. Top Performers Selection (100-200 best pure signals)
   ↓
3. Temporal Validation (mismas estrategias en período diferente)
   ↓
4. Emergent Risk Derivation (MAE/MFE analysis → optimal risk)
   ↓
5. Realistic Validation (Event-driven con emergent risk)
   ↓
6. Production Deployment (Ready-to-trade strategies)
```

## Justificación

1. **Evita Overfitting**: Evalúa calidad intrínseca de señales primero
2. **Flexibilidad**: Risk management adaptado a cada estrategia
3. **Robustez Temporal**: Validación en múltiples períodos
4. **Mejor Selección**: No descarta estrategias por risk management incorrecto

## Consecuencias

### Positivas
- ✅ Mejor calidad de estrategias seleccionadas
- ✅ Risk management optimizado por estrategia
- ✅ Menor overfitting temporal
- ✅ Más robustez

### Negativas
- ⚠️ Pipeline más complejo (3 fases vs 1)
- ⚠️ Más tiempo de procesamiento
- ⚠️ Requiere implementación de análisis MAE/MFE

## Implementación

Ver [Risk Management Emergente v2.2](../specifications/complete.md#risk-management-emergente) en especificaciones completas.

## Referencias

- `doc/risk_management_emergente_v2.2.md` (consolidado)
- `doc/risk_management_emergente.md` (consolidado)

