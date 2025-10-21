//! Repositorio de resultados de backtest

use crate::models::BacktestResult;
use sqlx::{Pool, Sqlite};

pub struct BacktestRepository {
    pool: Pool<Sqlite>,
}

impl BacktestRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// Crea un nuevo resultado de backtest
    pub async fn create(&self, result: &BacktestResult) -> Result<i64, sqlx::Error> {
        let row = sqlx::query(
            r#"
            INSERT INTO backtest_results 
            (strategy_id, dataset, timeframe, start_date, end_date, total_return, 
             sharpe_ratio, sortino_ratio, max_drawdown, win_rate, profit_factor, total_trades)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(result.strategy_id)
        .bind(&result.dataset)
        .bind(&result.timeframe)
        .bind(&result.start_date)
        .bind(&result.end_date)
        .bind(result.total_return)
        .bind(result.sharpe_ratio)
        .bind(result.sortino_ratio)
        .bind(result.max_drawdown)
        .bind(result.win_rate)
        .bind(result.profit_factor)
        .bind(result.total_trades)
        .execute(&self.pool)
        .await?;

        Ok(row.last_insert_rowid())
    }

    /// Busca resultados de backtest por estrategia
    pub async fn find_by_strategy(
        &self,
        strategy_id: i64,
    ) -> Result<Vec<BacktestResult>, sqlx::Error> {
        sqlx::query_as::<_, BacktestResult>(
            "SELECT * FROM backtest_results WHERE strategy_id = ? ORDER BY tested_at DESC"
        )
        .bind(strategy_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Obtiene el último resultado de backtest de una estrategia
    pub async fn get_latest(
        &self,
        strategy_id: i64,
    ) -> Result<Option<BacktestResult>, sqlx::Error> {
        sqlx::query_as::<_, BacktestResult>(
            "SELECT * FROM backtest_results WHERE strategy_id = ? ORDER BY tested_at DESC LIMIT 1"
        )
        .bind(strategy_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Lista los mejores resultados por Sharpe Ratio
    pub async fn top_by_sharpe(&self, limit: i32) -> Result<Vec<BacktestResult>, sqlx::Error> {
        sqlx::query_as::<_, BacktestResult>(
            "SELECT * FROM backtest_results ORDER BY sharpe_ratio DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    /// Cuenta resultados de backtest
    pub async fn count(&self) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM backtest_results")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Strategy;
    use crate::repositories::StrategyRepository;

    #[tokio::test]
    async fn test_create_and_find() {
        let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        // Crear estrategia primero
        let strat_repo = StrategyRepository::new(pool.clone());
        let strategy = Strategy::new(
            "Test".to_string(),
            "code".to_string(),
            "rust".to_string(),
        );
        let strategy_id = strat_repo.create(&strategy).await.unwrap();

        // Crear backtest result
        let repo = BacktestRepository::new(pool);
        let result = BacktestResult {
            id: None,
            strategy_id,
            dataset: "BTCUSDT".to_string(),
            timeframe: "1h".to_string(),
            start_date: "2024-01-01".to_string(),
            end_date: "2024-12-31".to_string(),
            total_return: 25.5,
            sharpe_ratio: 2.3,
            sortino_ratio: Some(2.8),
            max_drawdown: -15.0,
            win_rate: 65.0,
            profit_factor: Some(2.1),
            total_trades: 100,
            tested_at: None,
        };

        let id = repo.create(&result).await.unwrap();
        let results = repo.find_by_strategy(strategy_id).await.unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].sharpe_ratio, 2.3);
    }
}