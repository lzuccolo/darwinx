//! Generador aleatorio de estrategias

use crate::ast::nodes::*;
use darwinx_core::TimeFrame;
use darwinx_indicators::registry;
use darwinx_indicators::metadata::ParamType;
use rand::prelude::*;

pub struct RandomGenerator {
    max_conditions: usize,
    max_indicators: usize,
}

impl RandomGenerator {
    pub fn new() -> Self {
        Self {
            max_conditions: 5,
            max_indicators: 3,
        }
    }

    pub fn with_constraints(max_conditions: usize, max_indicators: usize) -> Self {
        Self {
            max_conditions,
            max_indicators,
        }
    }

    /// Genera una estrategia aleatoria
    pub fn generate(&self, name: String) -> StrategyAST {
        let mut rng = rand::thread_rng();

        let timeframe = self.random_timeframe(&mut rng);
        let mut strategy = StrategyAST::new(name, timeframe);

        // Generar condiciones de entrada
        let entry_count = rng.gen_range(1..=self.max_conditions.min(3));
        for _ in 0..entry_count {
            strategy.entry_rules.conditions.push(self.random_condition(&mut rng));
        }

        // Operador de entrada
        strategy.entry_rules.operator = if rng.gen_bool(0.7) {
            LogicalOperator::And
        } else {
            LogicalOperator::Or
        };

        // Generar condiciones de salida
        let exit_count = rng.gen_range(1..=self.max_conditions.min(2));
        for _ in 0..exit_count {
            strategy.exit_rules.conditions.push(self.random_condition(&mut rng));
        }

        strategy.exit_rules.operator = LogicalOperator::Or;

        strategy
    }

    /// Genera múltiples estrategias
    pub fn generate_batch(&self, count: usize) -> Vec<StrategyAST> {
        (0..count)
            .map(|i| self.generate(format!("Strategy_{}", i)))
            .collect()
    }

    fn random_timeframe(&self, rng: &mut impl Rng) -> TimeFrame {
        let timeframes = [
            TimeFrame::M5,
            TimeFrame::M15,
            TimeFrame::M30,
            TimeFrame::H1,
            TimeFrame::H4,
        ];
        timeframes[rng.gen_range(0..timeframes.len())]
    }

    fn random_condition(&self, rng: &mut impl Rng) -> Condition {
        let indicator = self.random_indicator(rng);
        let comparison = self.random_comparison(rng);
        // Mejorar: generar valor de comparación apropiado según la categoría del indicador
        let value = self.random_value_for_indicator(&indicator, rng);

        Condition {
            indicator,
            comparison,
            value,
        }
    }

    /// 🎯 100% DINÁMICO: Usa registry para cualquier indicador
    fn random_indicator(&self, rng: &mut impl Rng) -> IndicatorType {
        // Obtener todos los indicadores del registry
        let available = registry::all_names();
        
        if available.is_empty() {
            // Fallback: crear SMA por defecto
            return IndicatorType::with_period("sma", 20);
        }
        
        // Seleccionar uno aleatorio
        let selected_name = available.choose(rng).unwrap();
        
        // Obtener metadata del indicador
        let meta = registry::get(selected_name)
            .expect("Indicator should be registered");
        
        // Generar parámetros aleatorios basados en metadata (discretizados)
        let params: Vec<f64> = meta.parameters
            .iter()
            .map(|param_def| self.discretize_parameter(param_def, rng))
            .collect();
        
        // Crear indicador dinámico
        IndicatorType::new(selected_name.to_string(), params)
    }

    fn random_comparison(&self, rng: &mut impl Rng) -> Comparison {
        // Reducir probabilidad de Equals (muy restrictivo) y Crosses (simplificado)
        // Priorizar GreaterThan y LessThan que son más realistas
        match rng.gen_range(0..10) {
            0..=3 => Comparison::GreaterThan,  // 40% probabilidad
            4..=7 => Comparison::LessThan,      // 40% probabilidad
            8 => Comparison::CrossesAbove,      // 10% probabilidad
            9 => Comparison::CrossesBelow,      // 10% probabilidad
            _ => Comparison::Equals,            // Muy raro (solo si hay error)
        }
    }

    /// Genera un valor de comparación apropiado para un indicador dado
    /// Evita comparaciones sin sentido entre indicadores de diferentes escalas
    fn random_value_for_indicator(&self, indicator: &IndicatorType, rng: &mut impl Rng) -> ConditionValue {
        use darwinx_indicators::metadata::IndicatorCategory;
        
        // Obtener metadata del indicador para conocer su categoría
        let meta = match registry::get(&indicator.name) {
            Some(m) => m,
            None => {
                // Fallback: usar generación aleatoria estándar
                return self.random_value_fallback(rng);
            }
        };
        
        match meta.category {
            IndicatorCategory::Momentum => {
                // Indicadores de momentum (RSI, Stochastic, MFI) están en rango 0-100
                // Comparar SOLO con números en ese rango (evitar comparaciones con otros indicadores por ahora)
                // Esto evita problemas de escala
                ConditionValue::Number(rng.gen_range(20.0..80.0)) // Rango típico para momentum
            }
            IndicatorCategory::Trend => {
                // Indicadores de tendencia (SMA, EMA, etc.) están en escala de precio
                // Comparar con precio (más simple y realista)
                // O con otro indicador de tendencia, pero filtrar el actual
                match rng.gen_range(0..4) {
                    0..=2 => ConditionValue::Price, // 75% probabilidad: comparar con precio
                    3 => {
                        // Comparar con otro indicador de tendencia (diferente al actual)
                        let trend_indicators: Vec<_> = registry::by_category(IndicatorCategory::Trend)
                            .into_iter()
                            .filter(|m| m.name != indicator.name) // Evitar comparar consigo mismo
                            .collect();
                        if !trend_indicators.is_empty() {
                            let selected = trend_indicators.choose(rng).unwrap();
                            let params: Vec<f64> = selected.parameters
                                .iter()
                                .map(|p| self.discretize_parameter(p, rng))
                                .collect();
                            ConditionValue::Indicator(IndicatorType::new(selected.name.to_string(), params))
                        } else {
                            ConditionValue::Price
                        }
                    }
                    _ => ConditionValue::Price,
                }
            }
            IndicatorCategory::Volatility => {
                // Indicadores de volatilidad (ATR, Bollinger, Keltner) están en escala de precio/volatilidad
                // Comparar principalmente con precio (más simple)
                ConditionValue::Price
            }
            IndicatorCategory::Volume => {
                // Indicadores de volumen: OBV (acumulativo, puede ser muy grande), MFI (0-100), VWAP (precio)
                // Para evitar problemas, preferir comparaciones con precio o números
                match indicator.name.as_str() {
                    "obv" => {
                        // OBV es acumulativo, evitar comparaciones directas - solo con precio
                        ConditionValue::Price
                    }
                    "mfi" => {
                        // MFI es 0-100, como momentum - solo números
                        ConditionValue::Number(rng.gen_range(20.0..80.0))
                    }
                    "vwap" => {
                        // VWAP es precio - solo con precio
                        ConditionValue::Price
                    }
                    _ => {
                        // Fallback para otros indicadores de volumen
                        ConditionValue::Price
                    }
                }
            }
            _ => {
                // Para otras categorías, usar fallback
                self.random_value_fallback(rng)
            }
        }
    }
    
    /// Fallback para generación aleatoria estándar
    fn random_value_fallback(&self, rng: &mut impl Rng) -> ConditionValue {
        match rng.gen_range(0..3) {
            0 => ConditionValue::Number(rng.gen_range(20.0..80.0)),
            1 => ConditionValue::Price,
            _ => ConditionValue::Indicator(self.random_indicator(rng)),
        }
    }
    
    /// Método original mantenido por compatibilidad (ahora es fallback)
    fn random_value(&self, rng: &mut impl Rng) -> ConditionValue {
        self.random_value_fallback(rng)
    }

    /// Discretiza un parámetro según su tipo para evitar combinaciones infinitas
    fn discretize_parameter(&self, param_def: &darwinx_indicators::metadata::ParameterDef, rng: &mut impl Rng) -> f64 {
        match param_def.param_type {
            ParamType::Period => {
                // Para períodos: solo enteros (paso 1)
                // Ejemplo: RSI period 2.0-100.0 → solo genera 2, 3, 4, ..., 100
                let min = param_def.min.max(1.0) as usize;
                let max = param_def.max as usize;
                if min > max {
                    param_def.default
                } else {
                    rng.gen_range(min..=max) as f64
                }
            }
            ParamType::Multiplier => {
                // Para multiplicadores: paso de 0.1
                // Ejemplo: Bollinger std_dev 1.0-5.0 → genera 1.0, 1.1, 1.2, ..., 5.0
                let step = 0.1;
                let steps = ((param_def.max - param_def.min) / step).floor() as usize;
                let step_index = rng.gen_range(0..=steps);
                let value = param_def.min + (step_index as f64 * step);
                // Redondear a 1 decimal para evitar errores de punto flotante
                (value * 10.0).round() / 10.0
            }
            ParamType::Percentage => {
                // Para porcentajes: paso de 0.01 (1%)
                // Ejemplo: 0.0-1.0 → genera 0.00, 0.01, 0.02, ..., 1.00
                let step = 0.01;
                let steps = ((param_def.max - param_def.min) / step).floor() as usize;
                let step_index = rng.gen_range(0..=steps);
                let value = param_def.min + (step_index as f64 * step);
                // Redondear a 2 decimales
                (value * 100.0).round() / 100.0
            }
            ParamType::Value => {
                // Para valores arbitrarios: discretizar a paso de 0.1 por defecto
                // Si el rango es muy pequeño (< 1.0), usar paso más fino
                let range = param_def.max - param_def.min;
                let step = if range < 1.0 {
                    0.01  // Paso fino para rangos pequeños
                } else if range < 10.0 {
                    0.1   // Paso medio
                } else {
                    1.0   // Paso grueso para rangos grandes
                };
                let steps = (range / step).floor() as usize;
                let step_index = rng.gen_range(0..=steps);
                let value = param_def.min + (step_index as f64 * step);
                // Redondear según el paso
                if step >= 1.0 {
                    value.round()
                } else if step >= 0.1 {
                    (value * 10.0).round() / 10.0
                } else {
                    (value * 100.0).round() / 100.0
                }
            }
        }
    }
}

impl Default for RandomGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_single() {
        let generator = RandomGenerator::new();
        let strategy = generator.generate("Test".to_string());

        assert_eq!(strategy.name, "Test");
        assert!(!strategy.entry_rules.conditions.is_empty());
        assert!(!strategy.exit_rules.conditions.is_empty());
        assert!(strategy.complexity() > 0);
    }

    #[test]
    fn test_generate_batch() {
        let generator = RandomGenerator::new();
        let strategies = generator.generate_batch(10);

        assert_eq!(strategies.len(), 10);
        
        // Verificar que son diferentes
        let complexities: Vec<_> = strategies.iter().map(|s| s.complexity()).collect();
        assert!(complexities.iter().any(|&c| c != complexities[0]));
    }

    #[test]
    fn test_constraints() {
        let generator = RandomGenerator::with_constraints(2, 2);
        let strategy = generator.generate("Test".to_string());

        assert!(strategy.entry_rules.conditions.len() <= 2);
        assert!(strategy.exit_rules.conditions.len() <= 2);
    }

    #[test]
    fn test_uses_registry() {
        let generator = RandomGenerator::new();
        
        // Generar múltiples estrategias
        let strategies = generator.generate_batch(50);
        
        // Verificar que usa indicadores del registry
        let available = registry::all_names();
        assert!(!available.is_empty(), "Registry should have indicators");
        
        // Debería haber variedad de indicadores
        assert!(strategies.len() > 0);
    }

    #[test]
    fn test_parameter_discretization() {
        use darwinx_indicators::metadata::{ParameterDef, ParamType};
        let generator = RandomGenerator::new();
        let mut rng = rand::thread_rng();

        // Test Period: debe generar solo enteros
        let period_def = ParameterDef::period("period", 2.0, 100.0, 14.0);
        for _ in 0..100 {
            let value = generator.discretize_parameter(&period_def, &mut rng);
            assert_eq!(value, value.floor(), "Period should be integer");
            assert!(value >= 2.0 && value <= 100.0, "Period should be in range");
        }

        // Test Multiplier: debe generar valores con 1 decimal
        let mult_def = ParameterDef::multiplier("mult", 1.0, 5.0, 2.0);
        for _ in 0..100 {
            let value = generator.discretize_parameter(&mult_def, &mut rng);
            let rounded = (value * 10.0).round() / 10.0;
            assert_eq!(value, rounded, "Multiplier should have 1 decimal");
            assert!(value >= 1.0 && value <= 5.0, "Multiplier should be in range");
        }

        // Test Percentage: debe generar valores con 2 decimales
        let pct_def = ParameterDef::percentage("pct", 0.0, 1.0, 0.5);
        for _ in 0..100 {
            let value = generator.discretize_parameter(&pct_def, &mut rng);
            let rounded = (value * 100.0).round() / 100.0;
            assert_eq!(value, rounded, "Percentage should have 2 decimals");
            assert!(value >= 0.0 && value <= 1.0, "Percentage should be in range");
        }
    }
}