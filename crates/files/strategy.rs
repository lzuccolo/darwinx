//! Constraints básicos para estrategias multi-timeframe
//!
//! ## Actualizaciones v2.1:
//! - ✨ max_timeframes constraint para multi-timeframe strategies
//! - ✨ Enhanced defaults para multi-TF workflows
//! - ✨ Presets optimizados para diferentes casos de uso

/// Constraints básicos para generación y validación de estrategias
#[derive(Debug, Clone)]
pub struct StrategyConstraints {
    /// Máximo número de condiciones (entry + exit)
    pub max_conditions: usize,
    
    /// Máximo número de indicadores únicos
    pub max_indicators: usize,
    
    /// ✨ NEW: Máximo número de timeframes utilizados
    pub max_timeframes: usize,
}

impl StrategyConstraints {
    /// Constructor personalizado
    pub fn new(max_conditions: usize, max_indicators: usize, max_timeframes: usize) -> Self {
        Self {
            max_conditions,
            max_indicators,
            max_timeframes,
        }
    }

    /// ✨ UPDATED: Constraints estrictas (estrategias simples, single timeframe)
    pub fn strict() -> Self {
        Self {
            max_conditions: 3,
            max_indicators: 2,
            max_timeframes: 1,  // Single timeframe only
        }
    }

    /// ✨ UPDATED: Constraints moderadas (balance, basic multi-timeframe)
    pub fn moderate() -> Self {
        Self {
            max_conditions: 6,    // Increased for multi-TF
            max_indicators: 4,    // Increased for multi-TF
            max_timeframes: 2,    // Current + one higher TF
        }
    }

    /// ✨ UPDATED: Constraints relajadas (estrategias complejas multi-timeframe)
    pub fn relaxed() -> Self {
        Self {
            max_conditions: 10,
            max_indicators: 6,
            max_timeframes: 3,   // All timeframe categories
        }
    }

    /// ✨ NEW: Constraints para screening masivo (generación automatizada)
    pub fn massive_screening() -> Self {
        Self {
            max_conditions: 8,
            max_indicators: 5,
            max_timeframes: 3,
        }
    }

    /// ✨ NEW: Constraints para estrategias profesionales (trading en vivo)
    pub fn professional() -> Self {
        Self {
            max_conditions: 12,   // Strategies más complejas permitidas
            max_indicators: 8,    // Más indicadores para análisis profundo
            max_timeframes: 3,    // Full multi-timeframe support
        }
    }

    /// ✨ NEW: Constraints para backtesting masivo (optimización)
    pub fn backtest_optimization() -> Self {
        Self {
            max_conditions: 6,    // Balance speed vs complexity
            max_indicators: 4,    // Reasonable for batch processing
            max_timeframes: 2,    // Limit for computational efficiency
        }
    }

    /// ✨ NEW: Verifica si los constraints permiten multi-timeframe
    pub fn allows_multi_timeframe(&self) -> bool {
        self.max_timeframes > 1
    }

    /// ✨ NEW: Retorna la configuración como string legible
    pub fn display(&self) -> String {
        format!(
            "StrategyConstraints {{ conditions: {}, indicators: {}, timeframes: {} }}",
            self.max_conditions,
            self.max_indicators,
            self.max_timeframes
        )
    }

    /// ✨ NEW: Valida que otro conjunto de constraints es compatible
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self.max_conditions >= other.max_conditions &&
        self.max_indicators >= other.max_indicators &&
        self.max_timeframes >= other.max_timeframes
    }

    /// ✨ NEW: Retorna constraints más restrictivos entre dos
    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            max_conditions: self.max_conditions.min(other.max_conditions),
            max_indicators: self.max_indicators.min(other.max_indicators),
            max_timeframes: self.max_timeframes.min(other.max_timeframes),
        }
    }

    /// ✨ NEW: Retorna constraints menos restrictivos entre dos
    pub fn union(&self, other: &Self) -> Self {
        Self {
            max_conditions: self.max_conditions.max(other.max_conditions),
            max_indicators: self.max_indicators.max(other.max_indicators),
            max_timeframes: self.max_timeframes.max(other.max_timeframes),
        }
    }
}

impl Default for StrategyConstraints {
    /// ✨ UPDATED: Default ahora es moderate() para soportar basic multi-timeframe
    fn default() -> Self {
        Self::moderate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_constraints() {
        let constraints = StrategyConstraints::default();
        assert_eq!(constraints.max_conditions, 6);
        assert_eq!(constraints.max_indicators, 4);
        assert_eq!(constraints.max_timeframes, 2);
        assert!(constraints.allows_multi_timeframe());
    }

    #[test]
    fn test_strict_constraints() {
        let constraints = StrategyConstraints::strict();
        assert_eq!(constraints.max_conditions, 3);
        assert_eq!(constraints.max_timeframes, 1);
        assert!(!constraints.allows_multi_timeframe());
        assert!(constraints.max_conditions < StrategyConstraints::default().max_conditions);
    }

    #[test]
    fn test_relaxed_constraints() {
        let constraints = StrategyConstraints::relaxed();
        assert_eq!(constraints.max_conditions, 10);
        assert_eq!(constraints.max_timeframes, 3);
        assert!(constraints.allows_multi_timeframe());
        assert!(constraints.max_conditions > StrategyConstraints::default().max_conditions);
    }

    #[test]
    fn test_new_presets() {
        let massive = StrategyConstraints::massive_screening();
        assert!(massive.allows_multi_timeframe());
        assert_eq!(massive.max_timeframes, 3);

        let professional = StrategyConstraints::professional();
        assert!(professional.allows_multi_timeframe());
        assert!(professional.max_conditions >= massive.max_conditions);

        let backtest = StrategyConstraints::backtest_optimization();
        assert!(backtest.allows_multi_timeframe());
        assert_eq!(backtest.max_timeframes, 2);
    }

    #[test]
    fn test_compatibility() {
        let strict = StrategyConstraints::strict();
        let moderate = StrategyConstraints::moderate();
        let relaxed = StrategyConstraints::relaxed();

        // Relaxed should be compatible with moderate and strict
        assert!(relaxed.is_compatible_with(&moderate));
        assert!(relaxed.is_compatible_with(&strict));

        // Moderate should be compatible with strict
        assert!(moderate.is_compatible_with(&strict));

        // But not the reverse
        assert!(!strict.is_compatible_with(&moderate));
        assert!(!moderate.is_compatible_with(&relaxed));
    }

    #[test]
    fn test_intersection_and_union() {
        let strict = StrategyConstraints::strict();
        let relaxed = StrategyConstraints::relaxed();

        let intersection = strict.intersection(&relaxed);
        assert_eq!(intersection.max_conditions, strict.max_conditions);
        assert_eq!(intersection.max_timeframes, strict.max_timeframes);

        let union = strict.union(&relaxed);
        assert_eq!(union.max_conditions, relaxed.max_conditions);
        assert_eq!(union.max_timeframes, relaxed.max_timeframes);
    }

    #[test]
    fn test_display() {
        let constraints = StrategyConstraints::moderate();
        let display = constraints.display();
        assert!(display.contains("conditions: 6"));
        assert!(display.contains("indicators: 4"));
        assert!(display.contains("timeframes: 2"));
    }

    #[test]
    fn test_custom_constraints() {
        let custom = StrategyConstraints::new(15, 10, 4);
        assert_eq!(custom.max_conditions, 15);
        assert_eq!(custom.max_indicators, 10);
        assert_eq!(custom.max_timeframes, 4);
        assert!(custom.allows_multi_timeframe());
    }
}
