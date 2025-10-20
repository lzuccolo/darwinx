use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Error de parsing de timeframe
#[derive(Debug, Error)]
pub enum TimeFrameError {
    #[error("Timeframe inválido: {0}")]
    Invalid(String),
}

/// Timeframe de las velas
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeFrame {
    /// 1 minuto
    M1,
    /// 5 minutos
    M5,
    /// 15 minutos
    M15,
    /// 30 minutos
    M30,
    /// 1 hora
    H1,
    /// 4 horas
    H4,
    /// 1 día
    D1,
    /// 1 semana
    W1,
    /// 1 mes
    MN1,
}

impl TimeFrame {
    /// Retorna la duración en milisegundos
    pub fn to_millis(&self) -> i64 {
        match self {
            TimeFrame::M1 => 60 * 1000,
            TimeFrame::M5 => 5 * 60 * 1000,
            TimeFrame::M15 => 15 * 60 * 1000,
            TimeFrame::M30 => 30 * 60 * 1000,
            TimeFrame::H1 => 60 * 60 * 1000,
            TimeFrame::H4 => 4 * 60 * 60 * 1000,
            TimeFrame::D1 => 24 * 60 * 60 * 1000,
            TimeFrame::W1 => 7 * 24 * 60 * 60 * 1000,
            TimeFrame::MN1 => 30 * 24 * 60 * 60 * 1000, // Aproximado
        }
    }

    /// Retorna la duración en segundos
    pub fn to_seconds(&self) -> i64 {
        self.to_millis() / 1000
    }

    /// Convierte a string para APIs
    pub fn as_str(&self) -> &str {
        match self {
            TimeFrame::M1 => "1m",
            TimeFrame::M5 => "5m",
            TimeFrame::M15 => "15m",
            TimeFrame::M30 => "30m",
            TimeFrame::H1 => "1h",
            TimeFrame::H4 => "4h",
            TimeFrame::D1 => "1d",
            TimeFrame::W1 => "1w",
            TimeFrame::MN1 => "1M",
        }
    }

    /// Parsea desde string
    pub fn from_str(s: &str) -> Result<Self, TimeFrameError> {
        match s.to_lowercase().as_str() {
            "1m" | "m1" => Ok(TimeFrame::M1),
            "5m" | "m5" => Ok(TimeFrame::M5),
            "15m" | "m15" => Ok(TimeFrame::M15),
            "30m" | "m30" => Ok(TimeFrame::M30),
            "1h" | "h1" => Ok(TimeFrame::H1),
            "4h" | "h4" => Ok(TimeFrame::H4),
            "1d" | "d1" => Ok(TimeFrame::D1),
            "1w" | "w1" => Ok(TimeFrame::W1),
            "1mo" | "mn1" => Ok(TimeFrame::MN1),
            _ => Err(TimeFrameError::Invalid(s.to_string())),
        }
    }

    /// Retorna todos los timeframes disponibles
    pub fn all() -> Vec<TimeFrame> {
        vec![
            TimeFrame::M1,
            TimeFrame::M5,
            TimeFrame::M15,
            TimeFrame::M30,
            TimeFrame::H1,
            TimeFrame::H4,
            TimeFrame::D1,
            TimeFrame::W1,
            TimeFrame::MN1,
        ]
    }
}

impl fmt::Display for TimeFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeframe_to_millis() {
        assert_eq!(TimeFrame::M1.to_millis(), 60_000);
        assert_eq!(TimeFrame::H1.to_millis(), 3_600_000);
        assert_eq!(TimeFrame::D1.to_millis(), 86_400_000);
    }

    #[test]
    fn test_timeframe_from_str() {
        assert_eq!(TimeFrame::from_str("1m").unwrap(), TimeFrame::M1);
        assert_eq!(TimeFrame::from_str("1h").unwrap(), TimeFrame::H1);
        assert_eq!(TimeFrame::from_str("1d").unwrap(), TimeFrame::D1);

        assert!(TimeFrame::from_str("invalid").is_err());
    }

    #[test]
    fn test_timeframe_display() {
        assert_eq!(TimeFrame::M1.to_string(), "1m");
        assert_eq!(TimeFrame::H4.to_string(), "4h");
    }

    #[test]
    fn test_timeframe_all() {
        let all = TimeFrame::all();
        assert_eq!(all.len(), 9);
        assert!(all.contains(&TimeFrame::M1));
        assert!(all.contains(&TimeFrame::MN1));
    }
}
