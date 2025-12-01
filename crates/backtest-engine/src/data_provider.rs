//! Data Provider trait para el Backtest Engine
//!
//! Este módulo define el trait DataProvider que permite al Backtest Engine
//! trabajar con diferentes fuentes de datos, incluyendo soporte multi-timeframe.

use async_trait::async_trait;
use darwinx_core::{Candle, TimeFrame};
use darwinx_data::MultiTimeframeContext;
use crate::error::BacktestError;

/// Trait para proveedores de datos (MTF-ready)
///
/// Permite al Backtest Engine trabajar con diferentes fuentes de datos:
/// - Single timeframe (datos de un solo timeframe)
/// - Multi-timeframe (datos de múltiples timeframes sincronizados)
#[async_trait]
pub trait DataProvider: Send + Sync {
    /// Obtiene la vela en un índice específico del timeframe principal
    async fn get_candle(&self, index: usize) -> Result<Option<&Candle>, BacktestError>;

    /// Obtiene el número total de velas disponibles
    async fn len(&self) -> Result<usize, BacktestError>;

    /// Obtiene el timeframe principal
    fn primary_timeframe(&self) -> TimeFrame;

    /// Obtiene datos de un timeframe específico (para MTF)
    async fn get_timeframe_data(
        &self,
        timeframe: &TimeFrame,
        index: usize,
    ) -> Result<Option<&Candle>, BacktestError>;

    /// Obtiene todos los timeframes disponibles
    fn available_timeframes(&self) -> Vec<TimeFrame>;
}

/// Provider para datos de un solo timeframe
pub struct SingleTimeFrameProvider {
    candles: Vec<Candle>,
    timeframe: TimeFrame,
}

impl SingleTimeFrameProvider {
    /// Crea un nuevo provider con datos de un solo timeframe
    pub fn new(candles: Vec<Candle>, timeframe: TimeFrame) -> Self {
        Self { candles, timeframe }
    }
}

#[async_trait]
impl DataProvider for SingleTimeFrameProvider {
    async fn get_candle(&self, index: usize) -> Result<Option<&Candle>, BacktestError> {
        Ok(self.candles.get(index))
    }

    async fn len(&self) -> Result<usize, BacktestError> {
        Ok(self.candles.len())
    }

    fn primary_timeframe(&self) -> TimeFrame {
        self.timeframe
    }

    async fn get_timeframe_data(
        &self,
        timeframe: &TimeFrame,
        _index: usize,
    ) -> Result<Option<&Candle>, BacktestError> {
        if timeframe == &self.timeframe {
            // Para single timeframe, solo retornamos si es el mismo
            Ok(None)
        } else {
            Err(BacktestError::DataError(anyhow::anyhow!(
                "Timeframe {} no disponible. Solo disponible: {}",
                timeframe,
                self.timeframe
            )))
        }
    }

    fn available_timeframes(&self) -> Vec<TimeFrame> {
        vec![self.timeframe]
    }
}

/// Provider para datos multi-timeframe
pub struct MultiTimeFrameProvider {
    context: MultiTimeframeContext,
}

impl MultiTimeFrameProvider {
    /// Crea un nuevo provider con contexto multi-timeframe
    pub fn new(context: MultiTimeframeContext) -> Self {
        Self { context }
    }
}

#[async_trait]
impl DataProvider for MultiTimeFrameProvider {
    async fn get_candle(&self, index: usize) -> Result<Option<&Candle>, BacktestError> {
        let primary = self.context.primary().ok_or_else(|| {
            BacktestError::DataError(anyhow::anyhow!("No hay datos para el timeframe principal"))
        })?;

        Ok(primary.candles.get(index))
    }

    async fn len(&self) -> Result<usize, BacktestError> {
        let primary = self.context.primary().ok_or_else(|| {
            BacktestError::DataError(anyhow::anyhow!("No hay datos para el timeframe principal"))
        })?;

        Ok(primary.len())
    }

    fn primary_timeframe(&self) -> TimeFrame {
        // El contexto ya tiene el timeframe principal
        *self.context.timeframes().first().unwrap_or(&darwinx_core::TimeFrame::M1)
    }

    async fn get_timeframe_data(
        &self,
        timeframe: &TimeFrame,
        index: usize,
    ) -> Result<Option<&Candle>, BacktestError> {
        let data = self.context.get(timeframe).ok_or_else(|| {
            BacktestError::DataError(anyhow::anyhow!(
                "Timeframe {} no disponible",
                timeframe
            ))
        })?;

        Ok(data.candles.get(index))
    }

    fn available_timeframes(&self) -> Vec<TimeFrame> {
        self.context.timeframes()
    }
}

