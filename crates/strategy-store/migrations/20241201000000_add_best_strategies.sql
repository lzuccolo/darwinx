-- Migration: Add support for best strategies with full AST and execution metadata
-- This extends the existing schema to support storing complete strategy ASTs
-- and marking strategies as "best" for genetic algorithm feedback

-- Add columns to strategies table for AST storage and best flag
ALTER TABLE strategies ADD COLUMN strategy_ast_json TEXT;
ALTER TABLE strategies ADD COLUMN is_best INTEGER DEFAULT 0;
ALTER TABLE strategies ADD COLUMN execution_metadata TEXT; -- JSON with dataset, config, etc.

-- Add index for best strategies queries
CREATE INDEX IF NOT EXISTS idx_strategies_is_best ON strategies(is_best, sharpe_ratio DESC);

-- Add index for execution metadata queries (by dataset, timeframe, etc.)
CREATE INDEX IF NOT EXISTS idx_strategies_metadata ON strategies(execution_metadata);

-- Add hash column for deduplication (hash of strategy_ast_json)
ALTER TABLE strategies ADD COLUMN strategy_hash TEXT;
CREATE UNIQUE INDEX IF NOT EXISTS idx_strategies_hash ON strategies(strategy_hash);

-- Extend backtest_results with more metrics from BacktestMetrics
ALTER TABLE backtest_results ADD COLUMN annualized_return REAL;
ALTER TABLE backtest_results ADD COLUMN max_drawdown_percent REAL;
ALTER TABLE backtest_results ADD COLUMN total_profit REAL;
ALTER TABLE backtest_results ADD COLUMN total_loss REAL;
ALTER TABLE backtest_results ADD COLUMN max_consecutive_wins INTEGER;
ALTER TABLE backtest_results ADD COLUMN max_consecutive_losses INTEGER;
ALTER TABLE backtest_results ADD COLUMN trades_per_month REAL;
ALTER TABLE backtest_results ADD COLUMN trades_per_year REAL;
ALTER TABLE backtest_results ADD COLUMN stop_loss_exits INTEGER DEFAULT 0;
ALTER TABLE backtest_results ADD COLUMN take_profit_exits INTEGER DEFAULT 0;
ALTER TABLE backtest_results ADD COLUMN signal_exits INTEGER DEFAULT 0;
ALTER TABLE backtest_results ADD COLUMN end_of_data_exits INTEGER DEFAULT 0;
ALTER TABLE backtest_results ADD COLUMN composite_score REAL; -- Weighted score from ranking

-- Add index for composite score queries
CREATE INDEX IF NOT EXISTS idx_backtest_composite_score ON backtest_results(composite_score DESC);


