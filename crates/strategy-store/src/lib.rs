// ============================================================================
// crates/strategy-store/src/lib.rs
// ============================================================================
//! DarwinX Strategy Store - Sistema de persistencia

pub mod models;
pub mod repositories;

pub use models::{BacktestResult, Strategy, Trade};
pub use repositories::{BacktestRepository, StrategyRepository};

use sqlx::{Pool, Sqlite};
use std::path::Path;

pub async fn init_sqlite(path: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let pool = sqlx::SqlitePool::connect(&format!("sqlite:{}", path)).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}







// ============================================================================
// crates/strategy-store/src/repositories.rs
// ============================================================================
pub mod strategy_repo;
pub mod backtest_repo;

pub use strategy_repo::StrategyRepository;
pub use backtest_repo::BacktestRepository;

// ============================================================================
// crates/strategy-store/src/repositories/strategy_repo.rs
// ============================================================================
use crate::models::Strategy;
use sqlx::{Pool, Sqlite};

pub struct StrategyRepository {
    pool: Pool<Sqlite>,
}

impl StrategyRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, strategy: &Strategy) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            INSERT INTO strategies (name, description, source_code, format, parameters, complexity_score)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            strategy.name,
            strategy.description,
            strategy.source_code,
            strategy.format,
            strategy.parameters,
            strategy.complexity_score
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Strategy>, sqlx::Error> {
        sqlx::query_as!(
            Strategy,
            r#"SELECT * FROM strategies WHERE id = ?"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn list(&self, page: i32, page_size: i32) -> Result<Vec<Strategy>, sqlx::Error> {
        let offset = (page - 1) * page_size;
        sqlx::query_as!(
            Strategy,
            r#"SELECT * FROM strategies ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
            page_size,
            offset
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(r#"DELETE FROM strategies WHERE id = ?"#, id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn update_metrics(
        &self,
        id: i64,
        sharpe: f64,
        total_return: f64,
        max_dd: f64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE strategies SET sharpe_ratio = ?, total_return = ?, max_drawdown = ? WHERE id = ?"#,
            sharpe,
            total_return,
            max_dd,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

// ============================================================================
// crates/strategy-store/src/repositories/backtest_repo.rs
// ============================================================================
use crate::models::BacktestResult;
use sqlx::{Pool, Sqlite};

pub struct BacktestRepository {
    pool: Pool<Sqlite>,
}

impl BacktestRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, result: &BacktestResult) -> Result<i64, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            INSERT INTO backtest_results 
            (strategy_id, dataset, timeframe, start_date, end_date, total_return, 
             sharpe_ratio, sortino_ratio, max_drawdown, win_rate, profit_factor, total_trades)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            "#,
            result.strategy_id,
            result.dataset,
            result.timeframe,
            result.start_date,
            result.end_date,
            result.total_return,
            result.sharpe_ratio,
            result.sortino_ratio,
            result.max_drawdown,
            result.win_rate,
            result.profit_factor,
            result.total_trades
        )
        .execute(&self.pool)
        .await?;

        Ok(row.last_insert_rowid())
    }

    pub async fn find_by_strategy(
        &self,
        strategy_id: i64,
    ) -> Result<Vec<BacktestResult>, sqlx::Error> {
        sqlx::query_as!(
            BacktestResult,
            r#"SELECT * FROM backtest_results WHERE strategy_id = ? ORDER BY tested_at DESC"#,
            strategy_id
        )
        .fetch_all(&self.pool)
        .await
    }
}

// ============================================================================
// crates/strategy-store/migrations/001_initial_schema.sql
// ============================================================================
/*
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
    UNIQUE(strategy_id, dataset)
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

-- Indexes
CREATE INDEX idx_strategies_sharpe ON strategies(sharpe_ratio DESC);
CREATE INDEX idx_strategies_return ON strategies(total_return DESC);
CREATE INDEX idx_backtest_strategy ON backtest_results(strategy_id);
CREATE INDEX idx_trades_backtest ON trades(backtest_result_id);
*/