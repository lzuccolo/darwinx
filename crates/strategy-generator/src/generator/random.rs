//! Generador aleatorio de estrategias

use crate::ast::nodes::*;
use darwinx_core::TimeFrame;
use rand::Rng;

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
        let value = self.random_value(rng);

        Condition {
            indicator,
            comparison,
            value,
        }
    }

    fn random_indicator(&self, rng: &mut impl Rng) -> IndicatorType {
        match rng.gen_range(0..6) {
            0 => IndicatorType::Sma {
                period: rng.gen_range(5..=50),
            },
            1 => IndicatorType::Ema {
                period: rng.gen_range(5..=50),
            },
            2 => IndicatorType::Rsi {
                period: rng.gen_range(7..=21),
            },
            3 => IndicatorType::Macd {
                fast: 12,
                slow: 26,
                signal: 9,
            },
            4 => IndicatorType::BollingerBands {
                period: 20,
                std_dev: 2.0,
            },
            _ => IndicatorType::Atr {
                period: rng.gen_range(10..=20),
            },
        }
    }

    fn random_comparison(&self, rng: &mut impl Rng) -> Comparison {
        match rng.gen_range(0..5) {
            0 => Comparison::GreaterThan,
            1 => Comparison::LessThan,
            2 => Comparison::CrossesAbove,
            3 => Comparison::CrossesBelow,
            _ => Comparison::Equals,
        }
    }

    fn random_value(&self, rng: &mut impl Rng) -> ConditionValue {
        match rng.gen_range(0..3) {
            0 => ConditionValue::Number(rng.gen_range(20.0..80.0)),
            1 => ConditionValue::Price,
            _ => ConditionValue::Indicator(self.random_indicator(rng)),
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
}