//! Metadata de indicadores

use serde::{Deserialize, Serialize};

/// Categoría de indicador
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndicatorCategory {
    Trend,
    Momentum,
    Volatility,
    Volume,
    CandlePattern,
}

/// Tipo de input que necesita el indicador
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputType {
    /// Serie de precios (close, open, etc.)
    PriceSeries,
    /// Serie de velas completas
    CandleSeries,
    /// Múltiples series (precio + volumen, etc.)
    MultiSeries,
}

/// Tipo de parámetro
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParamType {
    /// Período (número de velas)
    Period,
    /// Multiplicador
    Multiplier,
    /// Porcentaje (0.0 - 1.0)
    Percentage,
    /// Valor arbitrario
    Value,
}

/// Definición de un parámetro
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDef {
    pub name: &'static str,
    pub param_type: ParamType,
    pub min: f64,
    pub max: f64,
    pub default: f64,
    pub description: &'static str,
}

impl ParameterDef {
    /// Constructor para período
    pub fn period(name: &'static str, min: f64, max: f64, default: f64) -> Self {
        Self {
            name,
            param_type: ParamType::Period,
            min,
            max,
            default,
            description: "Period in candles",
        }
    }

    /// Constructor para multiplicador
    pub fn multiplier(name: &'static str, min: f64, max: f64, default: f64) -> Self {
        Self {
            name,
            param_type: ParamType::Multiplier,
            min,
            max,
            default,
            description: "Multiplier value",
        }
    }

    /// Constructor para porcentaje
    pub fn percentage(name: &'static str, min: f64, max: f64, default: f64) -> Self {
        Self {
            name,
            param_type: ParamType::Percentage,
            min,
            max,
            default,
            description: "Percentage (0.0 - 1.0)",
        }
    }
}

/// Metadata de un indicador
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorMetadata {
    pub name: &'static str,
    pub category: IndicatorCategory,
    pub input_type: InputType,
    pub lookback: usize,
    pub parameters: Vec<ParameterDef>,
    pub description: &'static str,
}

impl IndicatorMetadata {
    /// Crea metadata con builder pattern
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            category: IndicatorCategory::Trend,
            input_type: InputType::PriceSeries,
            lookback: 1,
            parameters: Vec::new(),
            description: "",
        }
    }

    /// Define la categoría
    pub fn category(mut self, category: IndicatorCategory) -> Self {
        self.category = category;
        self
    }

    /// Define el tipo de input
    pub fn input_type(mut self, input_type: InputType) -> Self {
        self.input_type = input_type;
        self
    }

    /// Define cuántas velas necesita mirar atrás
    pub fn lookback(mut self, lookback: usize) -> Self {
        self.lookback = lookback;
        self
    }

    /// Agrega un parámetro
    pub fn parameter(mut self, param: ParameterDef) -> Self {
        self.parameters.push(param);
        self
    }

    /// Define la descripción
    pub fn description(mut self, description: &'static str) -> Self {
        self.description = description;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_pattern() {
        let metadata = IndicatorMetadata::new("sma")
            .category(IndicatorCategory::Trend)
            .input_type(InputType::PriceSeries)
            .lookback(1)
            .parameter(ParameterDef::period("period", 2.0, 200.0, 20.0))
            .description("Simple Moving Average");

        assert_eq!(metadata.name, "sma");
        assert_eq!(metadata.category, IndicatorCategory::Trend);
        assert_eq!(metadata.parameters.len(), 1);
    }

    #[test]
    fn test_parameter_constructors() {
        let period = ParameterDef::period("period", 2.0, 50.0, 14.0);
        assert_eq!(period.param_type, ParamType::Period);
        assert_eq!(period.default, 14.0);

        let mult = ParameterDef::multiplier("mult", 1.0, 5.0, 2.0);
        assert_eq!(mult.param_type, ParamType::Multiplier);
    }
}