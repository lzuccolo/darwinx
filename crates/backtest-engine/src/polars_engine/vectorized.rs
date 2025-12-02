//! Motor vectorizado de backtest usando Polars
//!
//! Este motor procesa múltiples velas simultáneamente usando operaciones
//! vectorizadas de Polars para máximo throughput.

use async_trait::async_trait;
use crate::data_provider::DataProvider;
use crate::error::BacktestError;
use crate::types::{BacktestResult, BacktestMetrics, Trade, EquityPoint, BacktestMetadata};
use crate::config::BacktestConfig;

/// Motor de backtest vectorizado usando Polars
pub struct PolarsBacktestEngine;

impl PolarsBacktestEngine {
    /// Crea un nuevo motor Polars
    pub fn new() -> Self {
        Self
    }
}

impl Default for PolarsBacktestEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait para estrategias que pueden ser evaluadas en el backtest
#[async_trait]
pub trait Strategy: Send + Sync {
    /// Nombre de la estrategia
    fn name(&self) -> &str;

    /// Evalúa si debe entrar en posición long
    async fn should_enter_long(
        &self,
        data_provider: &dyn DataProvider,
        index: usize,
    ) -> Result<bool, BacktestError>;

    /// Evalúa si debe salir de posición long
    async fn should_exit_long(
        &self,
        data_provider: &dyn DataProvider,
        index: usize,
    ) -> Result<bool, BacktestError>;

    /// Evalúa si debe entrar en posición short
    async fn should_enter_short(
        &self,
        data_provider: &dyn DataProvider,
        index: usize,
    ) -> Result<bool, BacktestError>;

    /// Evalúa si debe salir de posición short
    async fn should_exit_short(
        &self,
        data_provider: &dyn DataProvider,
        index: usize,
    ) -> Result<bool, BacktestError>;

    /// Calcula el tamaño de la posición
    fn position_size(&self, balance: f64, price: f64, config: &BacktestConfig) -> f64 {
        // Implementación por defecto: usa risk_per_trade
        let risk_amount = balance * config.risk_per_trade;
        risk_amount / price
    }
}

/// Trait para el motor de backtest
#[async_trait]
pub trait BacktestEngine: Send + Sync {
    /// Ejecuta un backtest individual
    async fn run_backtest(
        &self,
        strategy: &dyn Strategy,
        data_provider: &dyn DataProvider,
        config: &BacktestConfig,
    ) -> Result<BacktestResult, BacktestError>;
}

/// Estado interno de una posición durante el backtest
#[derive(Debug, Clone)]
struct Position {
    entry_timestamp: i64,
    entry_price: f64,
    size: f64,
    is_long: bool,
}

impl Position {
    fn new(entry_timestamp: i64, entry_price: f64, size: f64, is_long: bool) -> Self {
        Self {
            entry_timestamp,
            entry_price,
            size,
            is_long,
        }
    }
}

#[async_trait]
impl BacktestEngine for PolarsBacktestEngine {
    async fn run_backtest(
        &self,
        strategy: &dyn Strategy,
        data_provider: &dyn DataProvider,
        config: &BacktestConfig,
    ) -> Result<BacktestResult, BacktestError> {
        let data_len = data_provider.len().await?;
        if data_len == 0 {
            return Err(BacktestError::DataError(anyhow::anyhow!(
                "No data available for backtest"
            )));
        }

        // Estado del backtest
        let mut balance = config.initial_balance;
        let mut current_position: Option<Position> = None;
        let mut trades = Vec::new();
        let mut equity_curve = Vec::new();

        // Procesar cada vela
        for i in 0..data_len {
            let candle = data_provider
                .get_candle(i)
                .await?
                .ok_or_else(|| BacktestError::DataError(anyhow::anyhow!("Missing candle at index {}", i)))?;

            // Evaluar estrategia
            if let Some(ref position) = current_position {
                // Evaluar salida
                let should_exit = if position.is_long {
                    strategy.should_exit_long(data_provider, i).await?
                } else {
                    strategy.should_exit_short(data_provider, i).await?
                };

                if should_exit {
                    // Cerrar posición
                    let exit_price = self.apply_slippage(candle.close, position.is_long, config);
                    let trade = self.close_position(
                        position.clone(),
                        candle.timestamp,
                        exit_price,
                        balance,
                        config,
                    )?;
                    balance = trade.pnl + balance - trade.commission;
                    trades.push(trade);
                    current_position = None;
                }
            } else {
                // Evaluar entrada
                let should_enter_long = strategy.should_enter_long(data_provider, i).await?;
                let should_enter_short = strategy.should_enter_short(data_provider, i).await?;

                if should_enter_long && !should_enter_short {
                    // Abrir posición long
                    let entry_price = self.apply_slippage(candle.close, true, config);
                    let size = strategy.position_size(balance, entry_price, config);
                    let commission = config.calculate_commission(entry_price * size);

                    if balance >= (entry_price * size + commission) {
                        current_position = Some(Position::new(
                            candle.timestamp,
                            entry_price,
                            size,
                            true,
                        ));
                        balance -= commission;
                    }
                } else if should_enter_short && !should_enter_long {
                    // Abrir posición short
                    let entry_price = self.apply_slippage(candle.close, false, config);
                    let size = strategy.position_size(balance, entry_price, config);
                    let commission = config.calculate_commission(entry_price * size);

                    if balance >= commission {
                        current_position = Some(Position::new(
                            candle.timestamp,
                            entry_price,
                            size,
                            false,
                        ));
                        balance -= commission;
                    }
                }
            }

            // Registrar punto de equity
            let current_equity = if let Some(ref pos) = current_position {
                let current_price = candle.close;
                let unrealized_pnl = if pos.is_long {
                    (current_price - pos.entry_price) * pos.size
                } else {
                    (pos.entry_price - current_price) * pos.size
                };
                balance + unrealized_pnl
            } else {
                balance
            };

            equity_curve.push(EquityPoint {
                timestamp: candle.timestamp,
                balance: current_equity,
                drawdown: 0.0, // Se calculará después
            });
        }

        // Cerrar posición abierta al final si existe
        if let Some(position) = current_position {
            let last_candle = data_provider
                .get_candle(data_len - 1)
                .await?
                .ok_or_else(|| BacktestError::DataError(anyhow::anyhow!("Missing last candle")))?;
            let exit_price = self.apply_slippage(last_candle.close, position.is_long, config);
            let trade = self.close_position(
                position,
                last_candle.timestamp,
                exit_price,
                balance,
                config,
            )?;
            balance = trade.pnl + balance - trade.commission;
            trades.push(trade);
        }

        // Calcular métricas
        let metrics = self.calculate_metrics(
            &trades,
            &equity_curve,
            config.initial_balance,
            balance,
        )?;

        // Calcular drawdowns en equity curve
        let mut max_equity = config.initial_balance;
        for point in &mut equity_curve {
            if point.balance > max_equity {
                max_equity = point.balance;
            }
            point.drawdown = if max_equity > 0.0 {
                (max_equity - point.balance) / max_equity
            } else {
                0.0
            };
        }

        // Obtener primera y última vela para metadata
        let first_candle = data_provider
            .get_candle(0)
            .await?
            .ok_or_else(|| BacktestError::DataError(anyhow::anyhow!("Missing first candle")))?;
        let last_candle = data_provider
            .get_candle(data_len - 1)
            .await?
            .ok_or_else(|| BacktestError::DataError(anyhow::anyhow!("Missing last candle")))?;

        Ok(BacktestResult {
            strategy_name: strategy.name().to_string(),
            metrics,
            trades,
            equity_curve,
            metadata: BacktestMetadata {
                start_date: first_candle.timestamp,
                end_date: last_candle.timestamp,
                total_candles: data_len,
                initial_balance: config.initial_balance,
                final_balance: balance,
                config: config.clone(),
            },
        })
    }
}

impl PolarsBacktestEngine {
    /// Aplica slippage al precio
    fn apply_slippage(&self, price: f64, is_long: bool, config: &BacktestConfig) -> f64 {
        let slippage = config.calculate_slippage(price);
        if is_long {
            price + slippage // Compramos más caro
        } else {
            price - slippage // Vendemos más barato
        }
    }

    /// Cierra una posición y crea un trade
    fn close_position(
        &self,
        position: Position,
        exit_timestamp: i64,
        exit_price: f64,
        _current_balance: f64,
        config: &BacktestConfig,
    ) -> Result<Trade, BacktestError> {
        let trade_value = exit_price * position.size;
        let commission = config.calculate_commission(trade_value);
        let slippage = config.calculate_slippage(exit_price) * position.size;

        let pnl = if position.is_long {
            (exit_price - position.entry_price) * position.size - commission
        } else {
            (position.entry_price - exit_price) * position.size - commission
        };

        Ok(Trade {
            entry_timestamp: position.entry_timestamp,
            exit_timestamp,
            entry_price: position.entry_price,
            exit_price,
            size: position.size,
            is_long: position.is_long,
            pnl,
            commission,
            slippage,
            exit_reason: "Strategy signal".to_string(),
        })
    }

    /// Calcula todas las métricas del backtest
    fn calculate_metrics(
        &self,
        trades: &[Trade],
        equity_curve: &[EquityPoint],
        initial_balance: f64,
        final_balance: f64,
    ) -> Result<BacktestMetrics, BacktestError> {
        use crate::metrics::*;

        // Returns
        let total_return = calculate_total_return(initial_balance, final_balance);
        
        // Calcular días (aproximado desde timestamps)
        let days = if !equity_curve.is_empty() {
            let duration_ms = equity_curve.last().unwrap().timestamp - equity_curve[0].timestamp;
            duration_ms as f64 / (1000.0 * 60.0 * 60.0 * 24.0)
        } else {
            0.0
        };
        let annualized_return = calculate_annualized_return(total_return, days);

        // Calcular returns periódicos para Sharpe/Sortino
        let returns: Vec<f64> = equity_curve
            .windows(2)
            .map(|w| (w[1].balance - w[0].balance) / w[0].balance)
            .collect();
        let risk_free_rate = 0.0; // Asumir 0% por ahora
        let sharpe_ratio = calculate_sharpe_ratio(&returns, risk_free_rate);
        let sortino_ratio = calculate_sortino_ratio(&returns, risk_free_rate);

        // Risk
        let max_drawdown = calculate_max_drawdown(equity_curve);
        let max_drawdown_duration = calculate_max_drawdown_duration(equity_curve);
        let calmar_ratio = calculate_calmar_ratio(annualized_return, max_drawdown);
        let var_95 = calculate_var_95(&returns);

        // Statistics
        let total_trades = trades.len();
        let winning_trades = trades.iter().filter(|t| t.pnl > 0.0).count();
        let losing_trades = trades.iter().filter(|t| t.pnl < 0.0).count();
        let win_rate = calculate_win_rate(trades);
        let profit_factor = calculate_profit_factor(trades);
        let average_win = calculate_average_win(trades);
        let average_loss = calculate_average_loss(trades);
        let largest_win = calculate_largest_win(trades);
        let largest_loss = calculate_largest_loss(trades);
        let expectancy = calculate_expectancy(trades);
        
        let total_profit = calculate_total_profit(trades);
        let total_loss = calculate_total_loss(trades);
        let recovery_factor = calculate_recovery_factor(total_profit, max_drawdown * initial_balance);

        // Calcular duraciones de trades
        let average_trade_duration_ms = calculate_average_trade_duration(trades);
        let average_winning_trade_duration_ms = calculate_average_winning_trade_duration(trades);
        let average_losing_trade_duration_ms = calculate_average_losing_trade_duration(trades);

        // Calcular rachas
        let max_consecutive_wins = calculate_max_consecutive_wins(trades);
        let max_consecutive_losses = calculate_max_consecutive_losses(trades);

        // Calcular frecuencia de trading
        let trades_per_month = if days > 0.0 {
            (total_trades as f64 / days) * 30.0
        } else {
            0.0
        };
        let trades_per_year = if days > 0.0 {
            (total_trades as f64 / days) * 365.0
        } else {
            0.0
        };

        Ok(BacktestMetrics {
            total_return,
            annualized_return,
            sharpe_ratio,
            sortino_ratio,
            max_drawdown,
            max_drawdown_duration,
            calmar_ratio,
            var_95,
            total_trades,
            winning_trades,
            losing_trades,
            win_rate,
            profit_factor,
            average_win,
            average_loss,
            largest_win,
            largest_loss,
            expectancy,
            recovery_factor,
            average_trade_duration_ms,
            average_winning_trade_duration_ms,
            average_losing_trade_duration_ms,
            max_drawdown_percent: max_drawdown, // Ya está como fracción (0.0-1.0)
            total_profit,
            total_loss,
            max_consecutive_wins,
            max_consecutive_losses,
            trades_per_month,
            trades_per_year,
        })
    }
}

