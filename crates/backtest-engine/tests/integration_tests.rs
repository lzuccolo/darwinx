//! Tests de integración para el Backtest Engine

use darwinx_backtest_engine::*;
use darwinx_core::{Candle, TimeFrame};

/// Estrategia de prueba simple: compra y mantiene
struct BuyAndHoldStrategy {
    name: String,
}

impl BuyAndHoldStrategy {
    fn new() -> Self {
        Self {
            name: "BuyAndHold".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl Strategy for BuyAndHoldStrategy {
    fn name(&self) -> &str {
        &self.name
    }

    async fn should_enter_long(
        &self,
        _data_provider: &dyn DataProvider,
        index: usize,
    ) -> Result<bool, BacktestError> {
        // Entrar en la primera vela
        Ok(index == 0)
    }

    async fn should_exit_long(
        &self,
        _data_provider: &dyn DataProvider,
        _index: usize,
    ) -> Result<bool, BacktestError> {
        // Nunca salir (buy and hold)
        Ok(false)
    }

    async fn should_enter_short(
        &self,
        _data_provider: &dyn DataProvider,
        _index: usize,
    ) -> Result<bool, BacktestError> {
        Ok(false)
    }

    async fn should_exit_short(
        &self,
        _data_provider: &dyn DataProvider,
        _index: usize,
    ) -> Result<bool, BacktestError> {
        Ok(false)
    }
}

/// Estrategia de prueba: compra y vende después de 5 velas
struct SimpleReversalStrategy {
    name: String,
}

impl SimpleReversalStrategy {
    fn new() -> Self {
        Self {
            name: "SimpleReversal".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl Strategy for SimpleReversalStrategy {
    fn name(&self) -> &str {
        &self.name
    }

    async fn should_enter_long(
        &self,
        _data_provider: &dyn DataProvider,
        index: usize,
    ) -> Result<bool, BacktestError> {
        // Entrar cada 10 velas
        Ok(index % 10 == 0 && index > 0)
    }

    async fn should_exit_long(
        &self,
        _data_provider: &dyn DataProvider,
        index: usize,
    ) -> Result<bool, BacktestError> {
        // Salir después de 5 velas
        Ok(index % 10 == 5)
    }

    async fn should_enter_short(
        &self,
        _data_provider: &dyn DataProvider,
        _index: usize,
    ) -> Result<bool, BacktestError> {
        Ok(false)
    }

    async fn should_exit_short(
        &self,
        _data_provider: &dyn DataProvider,
        _index: usize,
    ) -> Result<bool, BacktestError> {
        Ok(false)
    }
}

fn create_test_candles(count: usize) -> Vec<Candle> {
    let base_timestamp = 1609459200000i64; // 2021-01-01
    let mut candles = Vec::new();

    for i in 0..count {
        let timestamp = base_timestamp + (i as i64 * 60_000); // 1 minuto por vela
        let price = 29000.0 + (i as f64 * 10.0); // Precio subiendo
        candles.push(Candle::new(
            timestamp,
            price,
            price + 100.0,
            price - 100.0,
            price + 50.0,
            1000.0 + (i as f64 * 10.0),
        ));
    }

    candles
}

#[tokio::test]
async fn test_buy_and_hold_strategy() {
    let engine = PolarsBacktestEngine::new();
    let strategy = BuyAndHoldStrategy::new();
    let candles = create_test_candles(100);
    let provider = SingleTimeFrameProvider::new(candles, TimeFrame::M1);
    let config = BacktestConfig::default();

    let result = engine
        .run_backtest(&strategy, &provider, &config)
        .await
        .unwrap();

    assert_eq!(result.strategy_name, "BuyAndHold");
    assert_eq!(result.trades.len(), 1); // 1 trade: cierre al final del backtest
    assert!(result.equity_curve.len() > 0);
    assert_eq!(result.metadata.total_candles, 100);
}

#[tokio::test]
async fn test_simple_reversal_strategy() {
    let engine = PolarsBacktestEngine::new();
    let strategy = SimpleReversalStrategy::new();
    let candles = create_test_candles(100);
    let provider = SingleTimeFrameProvider::new(candles, TimeFrame::M1);
    let config = BacktestConfig::default();

    let result = engine
        .run_backtest(&strategy, &provider, &config)
        .await
        .unwrap();

    assert_eq!(result.strategy_name, "SimpleReversal");
    assert!(result.trades.len() > 0); // Debería haber trades
    assert!(result.metrics.total_trades > 0);
    assert!(result.equity_curve.len() > 0);
}

#[tokio::test]
async fn test_metrics_calculation() {
    let engine = PolarsBacktestEngine::new();
    let strategy = SimpleReversalStrategy::new();
    let candles = create_test_candles(100);
    let provider = SingleTimeFrameProvider::new(candles, TimeFrame::M1);
    let config = BacktestConfig::default();

    let result = engine
        .run_backtest(&strategy, &provider, &config)
        .await
        .unwrap();

    // Verificar que las métricas están calculadas
    assert!(result.metrics.total_return.is_finite());
    assert!(result.metrics.sharpe_ratio.is_finite());
    assert!(result.metrics.max_drawdown >= 0.0);
    assert!(result.metrics.max_drawdown <= 1.0);
    assert!(result.metrics.win_rate >= 0.0);
    assert!(result.metrics.win_rate <= 1.0);
}

#[tokio::test]
async fn test_empty_data() {
    let engine = PolarsBacktestEngine::new();
    let strategy = BuyAndHoldStrategy::new();
    let provider = SingleTimeFrameProvider::new(Vec::new(), TimeFrame::M1);
    let config = BacktestConfig::default();

    let result = engine.run_backtest(&strategy, &provider, &config).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        BacktestError::DataError(_) => {} // Esperado
        _ => panic!("Expected DataError"),
    }
}

