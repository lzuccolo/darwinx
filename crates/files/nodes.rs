//! Nodos del Abstract Syntax Tree para estrategias multi-timeframe
//!
//! ## Nuevas características v2.1:
//! - ✨ TimeframeCategory: Current/Medium/High relative timeframes
//! - ✨ Multi-timeframe IndicatorType con timeframe_category field
//! - ✨ Enhanced StrategyAST con primary_timeframe y multi-TF support
//! - ✨ Comprehensive timeframe analysis methods

use darwinx_core::TimeFrame;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// ✨ NEW: Categoría de timeframe relativa al timeframe principal
///
/// En lugar de timeframes absolutos, usamos categorías semánticas:
/// - Current: Timeframe principal de la estrategia
/// - Medium: 3-5x el timeframe principal
/// - High: 12-24x el timeframe principal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeframeCategory {
    /// Timeframe principal de la estrategia (ej: si strategy es 5m, Current = 5m)
    Current,
    /// Timeframe medio (3-5x principal, ej: si strategy es 5m, Medium = 15m)
    Medium,
    /// Timeframe alto (12-24x principal, ej: si strategy es 5m, High = 1h)
    High,
}

impl TimeframeCategory {
    /// Retorna todas las categorías disponibles
    pub fn all() -> Vec<Self> {
        vec![Self::Current, Self::Medium, Self::High]
    }

    /// Convierte la categoría a timeframe absoluto basado en el timeframe principal
    ///
    /// # Mapping table:
    /// | Principal | Current | Medium | High |
    /// |-----------|---------|--------|------|
    /// | 1m        | 1m      | 5m     | 1h   |
    /// | 5m        | 5m      | 15m    | 1h   |
    /// | 15m       | 15m     | 1h     | 4h   |
    /// | 1h        | 1h      | 4h     | 1d   |
    /// | 4h        | 4h      | 1d     | 1w   |
    /// | 1d        | 1d      | 1w     | 1M   |
    pub fn to_absolute_timeframe(&self, primary_timeframe: TimeFrame) -> TimeFrame {
        match (primary_timeframe, self) {
            // 1m principal
            (TimeFrame::M1, TimeframeCategory::Current) => TimeFrame::M1,
            (TimeFrame::M1, TimeframeCategory::Medium) => TimeFrame::M5,
            (TimeFrame::M1, TimeframeCategory::High) => TimeFrame::H1,
            
            // 5m principal  
            (TimeFrame::M5, TimeframeCategory::Current) => TimeFrame::M5,
            (TimeFrame::M5, TimeframeCategory::Medium) => TimeFrame::M15,
            (TimeFrame::M5, TimeframeCategory::High) => TimeFrame::H1,
            
            // 15m principal
            (TimeFrame::M15, TimeframeCategory::Current) => TimeFrame::M15,
            (TimeFrame::M15, TimeframeCategory::Medium) => TimeFrame::H1,
            (TimeFrame::M15, TimeframeCategory::High) => TimeFrame::H4,
            
            // 1h principal
            (TimeFrame::H1, TimeframeCategory::Current) => TimeFrame::H1,
            (TimeFrame::H1, TimeframeCategory::Medium) => TimeFrame::H4,
            (TimeFrame::H1, TimeframeCategory::High) => TimeFrame::D1,
            
            // 4h principal
            (TimeFrame::H4, TimeframeCategory::Current) => TimeFrame::H4,
            (TimeFrame::H4, TimeframeCategory::Medium) => TimeFrame::D1,
            (TimeFrame::H4, TimeframeCategory::High) => TimeFrame::W1,
            
            // 1d principal
            (TimeFrame::D1, TimeframeCategory::Current) => TimeFrame::D1,
            (TimeFrame::D1, TimeframeCategory::Medium) => TimeFrame::W1,
            (TimeFrame::D1, TimeframeCategory::High) => TimeFrame::MN1,
            
            // 1w principal
            (TimeFrame::W1, TimeframeCategory::Current) => TimeFrame::W1,
            (TimeFrame::W1, TimeframeCategory::Medium) => TimeFrame::MN1,
            (TimeFrame::W1, TimeframeCategory::High) => TimeFrame::MN1, // Same as medium
            
            // 1M principal
            (TimeFrame::MN1, TimeframeCategory::Current) => TimeFrame::MN1,
            (TimeFrame::MN1, TimeframeCategory::Medium) => TimeFrame::MN1, // Same as current
            (TimeFrame::MN1, TimeframeCategory::High) => TimeFrame::MN1,   // Same as current
        }
    }

    /// Retorna el nombre legible de la categoría
    pub fn display_name(&self) -> &'static str {
        match self {
            TimeframeCategory::Current => "Current",
            TimeframeCategory::Medium => "Medium",
            TimeframeCategory::High => "High",
        }
    }
}

/// ✨ UPDATED: Estrategia completa con soporte multi-timeframe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyAST {
    /// Nombre de la estrategia
    pub name: String,
    
    /// ✨ NEW: Timeframe principal de la estrategia
    /// Todas las categorías se calculan relativas a este timeframe
    pub primary_timeframe: TimeFrame,
    
    /// Reglas de entrada
    pub entry_rules: RuleSet,
    
    /// Reglas de salida
    pub exit_rules: RuleSet,
}

/// Conjunto de reglas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSet {
    pub operator: LogicalOperator,
    pub conditions: Vec<Condition>,
}

/// Operador lógico
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogicalOperator {
    And,
    Or,
}

/// Condición individual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub indicator: IndicatorType,
    pub comparison: Comparison,
    pub value: ConditionValue,
}

/// ✨ UPDATED: Indicador dinámico con soporte multi-timeframe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorType {
    /// Nombre del indicador (ej: "sma", "rsi", "macd")
    pub name: String,
    
    /// Parámetros del indicador en orden
    /// Ejemplo: 
    /// - SMA: [20.0] (period)
    /// - MACD: [12.0, 26.0, 9.0] (fast, slow, signal)
    /// - Bollinger: [20.0, 2.0] (period, std_dev)
    pub params: Vec<f64>,
    
    /// ✨ NEW: Categoría de timeframe relativa al timeframe principal
    pub timeframe_category: TimeframeCategory,
}

impl IndicatorType {
    /// Crea un nuevo indicador con timeframe category
    pub fn new(name: impl Into<String>, params: Vec<f64>, timeframe_category: TimeframeCategory) -> Self {
        Self {
            name: name.into(),
            params,
            timeframe_category,
        }
    }

    /// Constructor conveniente para indicadores con 1 parámetro (period)
    pub fn with_period(name: impl Into<String>, period: usize, timeframe_category: TimeframeCategory) -> Self {
        Self {
            name: name.into(),
            params: vec![period as f64],
            timeframe_category,
        }
    }

    /// ✨ DEPRECATED: Mantener backward compatibility
    /// Use `new()` with timeframe_category instead
    #[deprecated(since = "2.1.0", note = "Use new() with timeframe_category instead")]
    pub fn new_legacy(name: impl Into<String>, params: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            params,
            timeframe_category: TimeframeCategory::Current, // Default fallback
        }
    }

    /// ✨ DEPRECATED: Mantener backward compatibility
    /// Use `with_period()` with timeframe_category instead
    #[deprecated(since = "2.1.0", note = "Use with_period() with timeframe_category instead")]
    pub fn with_period_legacy(name: impl Into<String>, period: usize) -> Self {
        Self {
            name: name.into(),
            params: vec![period as f64],
            timeframe_category: TimeframeCategory::Current, // Default fallback
        }
    }
    
    /// Retorna el nombre del indicador
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Retorna los parámetros
    pub fn params(&self) -> &[f64] {
        &self.params
    }

    /// ✨ NEW: Retorna la categoría de timeframe
    pub fn timeframe_category(&self) -> TimeframeCategory {
        self.timeframe_category
    }

    /// ✨ NEW: Retorna el timeframe absoluto basado en el primary timeframe
    pub fn absolute_timeframe(&self, primary_timeframe: TimeFrame) -> TimeFrame {
        self.timeframe_category.to_absolute_timeframe(primary_timeframe)
    }

    /// ✨ NEW: Verifica si el indicador usa un timeframe específico
    pub fn uses_timeframe_category(&self, category: TimeframeCategory) -> bool {
        self.timeframe_category == category
    }
    
    /// Retorna una representación legible
    pub fn display(&self) -> String {
        let params_str = if self.params.is_empty() {
            String::new()
        } else {
            format!("({})", 
                self.params.iter()
                    .map(|p| format!("{:.1}", p))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };

        format!("{}{}@{}", self.name, params_str, self.timeframe_category.display_name())
    }

    /// ✨ NEW: Retorna el display name con timeframe absoluto
    pub fn display_with_absolute_timeframe(&self, primary_timeframe: TimeFrame) -> String {
        let params_str = if self.params.is_empty() {
            String::new()
        } else {
            format!("({})", 
                self.params.iter()
                    .map(|p| format!("{:.1}", p))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };

        let absolute_tf = self.absolute_timeframe(primary_timeframe);
        format!("{}{}@{:?}", self.name, params_str, absolute_tf)
    }
}

/// Comparación
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Comparison {
    GreaterThan,
    LessThan,
    CrossesAbove,
    CrossesBelow,
    Equals,
}

/// Valor de condición
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionValue {
    Number(f64),
    Indicator(IndicatorType),
    Price,
}

impl StrategyAST {
    /// ✨ UPDATED: Constructor con primary_timeframe
    pub fn new(name: String, primary_timeframe: TimeFrame) -> Self {
        Self {
            name,
            primary_timeframe,
            entry_rules: RuleSet {
                operator: LogicalOperator::And,
                conditions: Vec::new(),
            },
            exit_rules: RuleSet {
                operator: LogicalOperator::And,
                conditions: Vec::new(),
            },
        }
    }

    /// Retorna la complejidad de la estrategia
    pub fn complexity(&self) -> usize {
        self.entry_rules.conditions.len() + self.exit_rules.conditions.len()
    }

    /// ✨ NEW: Retorna todos los indicadores únicos en la estrategia
    pub fn all_indicators(&self) -> Vec<&IndicatorType> {
        let mut indicators = Vec::new();
        
        // Entry conditions
        for condition in &self.entry_rules.conditions {
            indicators.push(&condition.indicator);
            if let ConditionValue::Indicator(ind) = &condition.value {
                indicators.push(ind);
            }
        }
        
        // Exit conditions
        for condition in &self.exit_rules.conditions {
            indicators.push(&condition.indicator);
            if let ConditionValue::Indicator(ind) = &condition.value {
                indicators.push(ind);
            }
        }
        
        indicators
    }

    /// ✨ NEW: Retorna los timeframes utilizados en la estrategia
    pub fn used_timeframe_categories(&self) -> HashSet<TimeframeCategory> {
        let mut categories = HashSet::new();
        
        for indicator in self.all_indicators() {
            categories.insert(indicator.timeframe_category);
        }
        
        categories
    }

    /// ✨ NEW: Retorna mapping de categorías a timeframes absolutos
    pub fn timeframe_mapping(&self) -> HashMap<TimeframeCategory, TimeFrame> {
        let mut mapping = HashMap::new();
        
        for category in self.used_timeframe_categories() {
            mapping.insert(category, category.to_absolute_timeframe(self.primary_timeframe));
        }
        
        mapping
    }

    /// ✨ NEW: Verifica si la estrategia es multi-timeframe
    pub fn is_multi_timeframe(&self) -> bool {
        self.used_timeframe_categories().len() > 1
    }

    /// ✨ NEW: Retorna el número de timeframes únicos utilizados
    pub fn timeframe_count(&self) -> usize {
        self.used_timeframe_categories().len()
    }

    /// ✨ NEW: Retorna estadísticas de indicadores por timeframe
    pub fn indicator_stats_by_timeframe(&self) -> HashMap<TimeframeCategory, usize> {
        let mut stats = HashMap::new();
        
        for indicator in self.all_indicators() {
            *stats.entry(indicator.timeframe_category).or_insert(0) += 1;
        }
        
        stats
    }

    /// ✨ NEW: Retorna una representación legible de la estrategia
    pub fn display_summary(&self) -> String {
        let tf_mapping = self.timeframe_mapping();
        let stats = self.indicator_stats_by_timeframe();
        
        let mut summary = format!(
            "Strategy: {} (Primary: {:?})\n",
            self.name, self.primary_timeframe
        );
        
        if self.is_multi_timeframe() {
            summary.push_str("Multi-timeframe: ");
            for (category, count) in stats {
                let abs_tf = tf_mapping[&category];
                summary.push_str(&format!("{}@{:?}({}) ", category.display_name(), abs_tf, count));
            }
            summary.push('\n');
        } else {
            summary.push_str("Single timeframe\n");
        }
        
        summary.push_str(&format!(
            "Complexity: {} conditions ({} entry, {} exit)",
            self.complexity(),
            self.entry_rules.conditions.len(),
            self.exit_rules.conditions.len()
        ));
        
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeframe_category_mapping() {
        // Test 5m primary timeframe
        assert_eq!(
            TimeframeCategory::Current.to_absolute_timeframe(TimeFrame::M5),
            TimeFrame::M5
        );
        assert_eq!(
            TimeframeCategory::Medium.to_absolute_timeframe(TimeFrame::M5),
            TimeFrame::M15
        );
        assert_eq!(
            TimeframeCategory::High.to_absolute_timeframe(TimeFrame::M5),
            TimeFrame::H1
        );

        // Test 1h primary timeframe
        assert_eq!(
            TimeframeCategory::Current.to_absolute_timeframe(TimeFrame::H1),
            TimeFrame::H1
        );
        assert_eq!(
            TimeframeCategory::Medium.to_absolute_timeframe(TimeFrame::H1),
            TimeFrame::H4
        );
        assert_eq!(
            TimeframeCategory::High.to_absolute_timeframe(TimeFrame::H1),
            TimeFrame::D1
        );
    }

    #[test]
    fn test_strategy_creation() {
        let strategy = StrategyAST::new("Multi-TF Test".to_string(), TimeFrame::M5);
        assert_eq!(strategy.name, "Multi-TF Test");
        assert_eq!(strategy.primary_timeframe, TimeFrame::M5);
        assert_eq!(strategy.complexity(), 0);
        assert!(!strategy.is_multi_timeframe());
    }

    #[test]
    fn test_indicator_with_timeframe() {
        let indicator = IndicatorType::new("rsi", vec![14.0], TimeframeCategory::Medium);
        assert_eq!(indicator.name(), "rsi");
        assert_eq!(indicator.timeframe_category(), TimeframeCategory::Medium);
        assert_eq!(indicator.absolute_timeframe(TimeFrame::M5), TimeFrame::M15);
        assert_eq!(indicator.display(), "rsi(14.0)@Medium");
    }

    #[test]
    fn test_multi_timeframe_strategy() {
        let mut strategy = StrategyAST::new("Multi-TF Test".to_string(), TimeFrame::M5);
        
        // Add indicators from different timeframes
        strategy.entry_rules.conditions.push(Condition {
            indicator: IndicatorType::new("rsi", vec![14.0], TimeframeCategory::Current),
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Number(50.0),
        });
        
        strategy.entry_rules.conditions.push(Condition {
            indicator: IndicatorType::new("ema", vec![200.0], TimeframeCategory::Medium),
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Number(100.0),
        });

        assert!(strategy.is_multi_timeframe());
        assert_eq!(strategy.timeframe_count(), 2);
        
        let mapping = strategy.timeframe_mapping();
        assert_eq!(mapping[&TimeframeCategory::Current], TimeFrame::M5);
        assert_eq!(mapping[&TimeframeCategory::Medium], TimeFrame::M15);
    }

    #[test]
    fn test_indicator_display() {
        let rsi = IndicatorType::new("rsi", vec![14.0], TimeframeCategory::Current);
        assert_eq!(rsi.display(), "rsi(14.0)@Current");
        assert_eq!(rsi.display_with_absolute_timeframe(TimeFrame::M5), "rsi(14.0)@M5");
        
        let macd = IndicatorType::new("macd", vec![12.0, 26.0, 9.0], TimeframeCategory::High);
        assert_eq!(macd.display(), "macd(12.0, 26.0, 9.0)@High");
        
        let obv = IndicatorType::new("obv", vec![], TimeframeCategory::Medium);
        assert_eq!(obv.display(), "obv@Medium");
    }

    #[test]
    fn test_backward_compatibility() {
        #[allow(deprecated)]
        let indicator = IndicatorType::new_legacy("sma", vec![20.0]);
        assert_eq!(indicator.timeframe_category(), TimeframeCategory::Current);
        
        #[allow(deprecated)]
        let indicator2 = IndicatorType::with_period_legacy("rsi", 14);
        assert_eq!(indicator2.timeframe_category(), TimeframeCategory::Current);
    }

    #[test]
    fn test_strategy_summary() {
        let mut strategy = StrategyAST::new("Complex Multi-TF".to_string(), TimeFrame::H1);
        
        strategy.entry_rules.conditions.push(Condition {
            indicator: IndicatorType::new("rsi", vec![14.0], TimeframeCategory::Current),
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Number(50.0),
        });
        
        strategy.entry_rules.conditions.push(Condition {
            indicator: IndicatorType::new("ema", vec![200.0], TimeframeCategory::High),
            comparison: Comparison::GreaterThan,
            value: ConditionValue::Indicator(
                IndicatorType::new("sma", vec![50.0], TimeframeCategory::Medium)
            ),
        });

        let summary = strategy.display_summary();
        assert!(summary.contains("Complex Multi-TF"));
        assert!(summary.contains("Multi-timeframe"));
        assert!(summary.contains("H1"));
    }
}
