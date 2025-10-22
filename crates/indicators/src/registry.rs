//! Registry global de indicadores
//!
//! Sistema de auto-registro que permite descubrir indicadores dinámicamente
//! sin necesidad de mantener listas hardcodeadas.

use crate::metadata::{IndicatorCategory, IndicatorMetadata};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;

/// Registry thread-safe de indicadores
pub struct IndicatorRegistry {
    indicators: HashMap<&'static str, IndicatorMetadata>,
    by_category: HashMap<IndicatorCategory, Vec<&'static str>>,
}

impl IndicatorRegistry {
    fn new() -> Self {
        Self {
            indicators: HashMap::new(),
            by_category: HashMap::new(),
        }
    }

    /// Registra un nuevo indicador
    fn register(&mut self, meta: IndicatorMetadata) {
        let name = meta.name;
        let category = meta.category;

        self.indicators.insert(name, meta);
        self.by_category
            .entry(category)
            .or_insert_with(Vec::new)
            .push(name);
    }

    /// Obtiene metadata de un indicador por nombre
    fn get(&self, name: &str) -> Option<&IndicatorMetadata> {
        self.indicators.get(name)
    }

    /// Retorna todos los nombres de indicadores
    fn all_names(&self) -> Vec<&'static str> {
        self.indicators.keys().copied().collect()
    }

    /// Retorna todo el metadata de todos los indicadores
    fn all(&self) -> Vec<IndicatorMetadata> {
        self.indicators.values().cloned().collect()
    }

    /// Retorna indicadores de una categoría específica
    fn by_category(&self, category: IndicatorCategory) -> Vec<IndicatorMetadata> {
        self.by_category
            .get(&category)
            .map(|names| {
                names
                    .iter()
                    .filter_map(|name| self.get(name).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Retorna estadísticas del registry
    fn stats(&self) -> RegistryStats {
        let mut stats = RegistryStats::default();
        stats.total = self.indicators.len();

        for meta in self.indicators.values() {
            match meta.category {
                IndicatorCategory::Trend => stats.trend += 1,
                IndicatorCategory::Momentum => stats.momentum += 1,
                IndicatorCategory::Volatility => stats.volatility += 1,
                IndicatorCategory::Volume => stats.volume += 1,
                IndicatorCategory::CandlePattern => stats.candle_pattern += 1,
            }
        }

        stats
    }
}

#[derive(Debug, Default)]
pub struct RegistryStats {
    pub total: usize,
    pub trend: usize,
    pub momentum: usize,
    pub volatility: usize,
    pub volume: usize,
    pub candle_pattern: usize,
}

// Singleton global
static REGISTRY: Lazy<RwLock<IndicatorRegistry>> =
    Lazy::new(|| RwLock::new(IndicatorRegistry::new()));

// ============================================================================
// API PÚBLICA (Thread-safe)
// ============================================================================

/// Registra un indicador en el registry global
///
/// Esta función es llamada automáticamente por la macro `register_indicator!`
/// en tiempo de inicialización del programa.
pub fn register(meta: IndicatorMetadata) {
    REGISTRY.write().register(meta);
}

/// Obtiene metadata de un indicador por nombre
///
/// # Example
/// ```
/// use darwinx_indicators::registry;
///
/// if let Some(meta) = registry::get("sma") {
///     println!("SMA tiene {} parámetros", meta.parameters.len());
/// }
/// ```
pub fn get(name: &str) -> Option<IndicatorMetadata> {
    REGISTRY.read().get(name).cloned()
}

/// Retorna los nombres de todos los indicadores registrados
///
/// # Example
/// ```
/// use darwinx_indicators::registry;
///
/// let available = registry::all_names();
/// println!("Indicadores disponibles: {:?}", available);
/// ```
pub fn all_names() -> Vec<&'static str> {
    REGISTRY.read().all_names()
}

/// Retorna todo el metadata de todos los indicadores
pub fn all() -> Vec<IndicatorMetadata> {
    REGISTRY.read().all()
}

/// Retorna indicadores de una categoría específica
///
/// # Example
/// ```
/// use darwinx_indicators::{registry, metadata::IndicatorCategory};
///
/// let trend_indicators = registry::by_category(IndicatorCategory::Trend);
/// for meta in trend_indicators {
///     println!("Trend indicator: {}", meta.name);
/// }
/// ```
pub fn by_category(category: IndicatorCategory) -> Vec<IndicatorMetadata> {
    REGISTRY.read().by_category(category)
}

/// Retorna estadísticas del registry
///
/// # Example
/// ```
/// use darwinx_indicators::registry;
///
/// let stats = registry::stats();
/// println!("Total de indicadores: {}", stats.total);
/// println!("Trend: {}, Momentum: {}", stats.trend, stats.momentum);
/// ```
pub fn stats() -> RegistryStats {
    REGISTRY.read().stats()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_has_indicators() {
        let names = all_names();
        assert!(
            !names.is_empty(),
            "Registry should have at least one indicator"
        );
    }

    #[test]
    fn test_get_sma() {
        let meta = get("sma");
        assert!(meta.is_some(), "SMA should be registered");
        
        let meta = meta.unwrap();
        assert_eq!(meta.name, "sma");
        assert_eq!(meta.category, IndicatorCategory::Trend);
    }

    #[test]
    fn test_stats() {
        let stats = stats();
        println!("Registry stats: {:?}", stats);
        assert!(stats.total > 0);
        assert!(stats.trend > 0);
    }

    #[test]
    fn test_by_category() {
        let trend = by_category(IndicatorCategory::Trend);
        assert!(!trend.is_empty(), "Should have trend indicators");
        
        for meta in trend {
            assert_eq!(meta.category, IndicatorCategory::Trend);
        }
    }
}