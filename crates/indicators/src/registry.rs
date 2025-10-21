//! Registry global de indicadores

use crate::metadata::{IndicatorCategory, IndicatorMetadata};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Registry de indicadores
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

    /// Registra un indicador
    pub fn register(&mut self, metadata: IndicatorMetadata) {
        let name = metadata.name;
        let category = metadata.category;
        
        self.indicators.insert(name, metadata);
        self.by_category
            .entry(category)
            .or_insert_with(Vec::new)
            .push(name);
    }

    /// Obtiene metadata de un indicador por nombre
    pub fn get(&self, name: &str) -> Option<&IndicatorMetadata> {
        self.indicators.get(name)
    }

    /// Obtiene todos los indicadores de una categoría
    pub fn by_category(&self, category: IndicatorCategory) -> Vec<&IndicatorMetadata> {
        self.by_category
            .get(&category)
            .map(|names| {
                names
                    .iter()
                    .filter_map(|name| self.indicators.get(name))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Retorna todos los indicadores
    pub fn all(&self) -> Vec<&IndicatorMetadata> {
        self.indicators.values().collect()
    }

    /// Retorna un indicador aleatorio
    pub fn random(&self) -> Option<&IndicatorMetadata> {
        use rand::seq::SliceRandom;
        let all: Vec<_> = self.indicators.values().collect();
        all.choose(&mut rand::thread_rng()).copied()
    }

    /// Retorna un indicador aleatorio de una categoría
    pub fn random_from_category(&self, category: IndicatorCategory) -> Option<&IndicatorMetadata> {
        use rand::seq::SliceRandom;
        let indicators = self.by_category(category);
        indicators.choose(&mut rand::thread_rng()).copied()
    }

    /// Obtiene el registry global (singleton)
    pub fn global() -> &'static Self {
        static REGISTRY: OnceLock<IndicatorRegistry> = OnceLock::new();
        REGISTRY.get_or_init(build_registry)
    }
}

/// Construye el registry con todos los indicadores
fn build_registry() -> IndicatorRegistry {
    let mut registry = IndicatorRegistry::new();

    // Trend indicators
    registry.register(crate::trend::sma::metadata());
    registry.register(crate::trend::ema::metadata());
    registry.register(crate::trend::wma::metadata());
    registry.register(crate::trend::vwma::metadata());

    // Momentum indicators
    registry.register(crate::momentum::rsi::metadata());
    registry.register(crate::momentum::macd::metadata());
    registry.register(crate::momentum::roc::metadata());
    registry.register(crate::momentum::stochastic::metadata());

    // Volatility indicators
    registry.register(crate::volatility::atr::metadata());
    registry.register(crate::volatility::bollinger::metadata());
    registry.register(crate::volatility::keltner::metadata());

    // Volume indicators
    registry.register(crate::volume::obv::metadata());
    registry.register(crate::volume::mfi::metadata());
    registry.register(crate::volume::vwap::metadata());

    registry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_singleton() {
        let registry1 = IndicatorRegistry::global();
        let registry2 = IndicatorRegistry::global();
        
        // Mismo puntero (singleton)
        assert!(std::ptr::eq(registry1, registry2));
    }

    #[test]
    fn test_get_indicator() {
        let registry = IndicatorRegistry::global();
        let sma = registry.get("sma");
        
        assert!(sma.is_some());
        assert_eq!(sma.unwrap().name, "sma");
    }

    #[test]
    fn test_by_category() {
        let registry = IndicatorRegistry::global();
        let trend_indicators = registry.by_category(IndicatorCategory::Trend);
        
        assert!(!trend_indicators.is_empty());
        assert!(trend_indicators.iter().any(|i| i.name == "sma"));
    }

    #[test]
    fn test_random() {
        let registry = IndicatorRegistry::global();
        let random = registry.random();
        
        assert!(random.is_some());
    }
}