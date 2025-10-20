-- Initial schema for DarwinX Strategy Store

-- Strategies table
CREATE TABLE IF NOT EXISTS strategies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    source_code TEXT NOT NULL,
    format TEXT NOT NULL,
    parameters TEXT,
    sharpe_ratio REAL,
    total_return REAL,
    max_drawdown REAL,
    complexity_score REAL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Backtest results table
CREATE TABLE IF NOT EXISTS backtest_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    strategy_id INTEGER NOT NULL,
    dataset TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    total_return REAL NOT NULL,
    sharpe_ratio REAL NOT NULL,
    sortino_ratio REAL,
    max_drawdown REAL NOT NULL,
    win_rate REAL NOT NULL,
    profit_factor REAL,
    total_trades INTEGER NOT NULL,
    tested_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (strategy_id) REFERENCES strategies(id) ON DELETE CASCADE,
    UNIQUE(strategy_id, dataset, timeframe, start_date, end_date)
);

-- Trades table
CREATE TABLE IF NOT EXISTS trades (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    backtest_result_id INTEGER NOT NULL,
    entry_time TEXT NOT NULL,
    exit_time TEXT NOT NULL,
    side TEXT NOT NULL,
    entry_price REAL NOT NULL,
    exit_price REAL NOT NULL,
    quantity REAL NOT NULL,
    pnl REAL NOT NULL,
    pnl_percent REAL NOT NULL,
    FOREIGN KEY (backtest_result_id) REFERENCES backtest_results(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_strategies_sharpe ON strategies(sharpe_ratio DESC);
CREATE INDEX IF NOT EXISTS idx_strategies_return ON strategies(total_return DESC);
CREATE INDEX IF NOT EXISTS idx_strategies_created ON strategies(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_backtest_strategy ON backtest_results(strategy_id);
CREATE INDEX IF NOT EXISTS idx_backtest_sharpe ON backtest_results(sharpe_ratio DESC);
CREATE INDEX IF NOT EXISTS idx_trades_backtest ON trades(backtest_result_id);