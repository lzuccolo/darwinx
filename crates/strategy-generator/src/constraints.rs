//! Constraints y límites para estrategias

/// Constraints para generación y validación de estrategias
#[derive(Debug, Clone)]
pub struct StrategyConstraints {
    /// Máximo número de condiciones (entry + exit)
    pub max_conditions: usize,
    
    /// Máximo número de indicadores únicos
    pub max_indicators: usize,
    
    /// Máximo número de timeframes
    pub max_timeframes: usize,
}

impl StrategyConstraints {
    pub fn new(max_conditions: usize, max_indicators: usize, max_timeframes: usize) -> Self {
        Self {
            max_conditions,
            max_indicators,
            max_timeframes,
        }
    }

    /// Constraints estrictas (estrategias simples)
    pub fn strict() -> Self {
        Self {
            max_conditions: 3,
            max_indicators: 2,
            max_timeframes: 1,
        }
    }

    /// Constraints moderadas (balance)
    pub fn moderate() -> Self {
        Self {
            max_conditions: 5,
            max_indicators: 3,
            max_timeframes: 2,
        }
    }

    /// Constraints relajadas (estrategias complejas)
    pub fn relaxed() -> Self {
        Self {
            max_conditions: 10,
            max_indicators: 5,
            max_timeframes: 3,
        }
    }
}

impl Default for StrategyConstraints {
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
        assert_eq!(constraints.max_conditions, 5);
        assert_eq!(constraints.max_indicators, 3);
    }

    #[test]
    fn test_strict_constraints() {
        let constraints = StrategyConstraints::strict();
        assert_eq!(constraints.max_conditions, 3);
        assert!(constraints.max_conditions < StrategyConstraints::default().max_conditions);
    }

    #[test]
    fn test_relaxed_constraints() {
        let constraints = StrategyConstraints::relaxed();
        assert_eq!(constraints.max_conditions, 10);
        assert!(constraints.max_conditions > StrategyConstraints::default().max_conditions);
    }
}