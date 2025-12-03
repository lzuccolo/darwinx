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

    /// Crea un resultado de backtest con métricas extendidas
    pub async fn create_extended(&self, result: &BacktestResult) -> Result<i64, sqlx::Error> {
        let row = sqlx::query(
            r#"
            INSERT INTO backtest_results 
            (strategy_id, dataset, timeframe, start_date, end_date, total_return, 
             sharpe_ratio, sortino_ratio, max_drawdown, win_rate, profit_factor, total_trades,
             annualized_return, max_drawdown_percent, total_profit, total_loss,
             max_consecutive_wins, max_consecutive_losses, trades_per_month, trades_per_year,
             stop_loss_exits, take_profit_exits, signal_exits, end_of_data_exits, composite_score)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
        .bind(result.annualized_return)
        .bind(result.max_drawdown_percent)
        .bind(result.total_profit)
        .bind(result.total_loss)
        .bind(result.max_consecutive_wins)
        .bind(result.max_consecutive_losses)
        .bind(result.trades_per_month)
        .bind(result.trades_per_year)
        .bind(result.stop_loss_exits)
        .bind(result.take_profit_exits)
        .bind(result.signal_exits)
        .bind(result.end_of_data_exits)
        .bind(result.composite_score)
        .execute(&self.pool)
        .await?;

        Ok(row.last_insert_rowid())
    }

    /// Crea o actualiza un resultado de backtest con métricas extendidas
    /// Si ya existe un resultado para la misma combinación (strategy_id, dataset, timeframe, start_date, end_date),
    /// lo actualiza. Si no existe, lo crea.
    pub async fn create_or_update_extended(&self, result: &BacktestResult) -> Result<i64, sqlx::Error> {
        // Primero intentar buscar si ya existe
        let existing = sqlx::query_as::<_, BacktestResult>(
            r#"
            SELECT * FROM backtest_results 
            WHERE strategy_id = ? AND dataset = ? AND timeframe = ? AND start_date = ? AND end_date = ?
            LIMIT 1
            "#
        )
        .bind(result.strategy_id)
        .bind(&result.dataset)
        .bind(&result.timeframe)
        .bind(&result.start_date)
        .bind(&result.end_date)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(existing_result) = existing {
            // Actualizar resultado existente
            let _row = sqlx::query(
                r#"
                UPDATE backtest_results SET
                    total_return = ?,
                    sharpe_ratio = ?,
                    sortino_ratio = ?,
                    max_drawdown = ?,
                    win_rate = ?,
                    profit_factor = ?,
                    total_trades = ?,
                    annualized_return = ?,
                    max_drawdown_percent = ?,
                    total_profit = ?,
                    total_loss = ?,
                    max_consecutive_wins = ?,
                    max_consecutive_losses = ?,
                    trades_per_month = ?,
                    trades_per_year = ?,
                    stop_loss_exits = ?,
                    take_profit_exits = ?,
                    signal_exits = ?,
                    end_of_data_exits = ?,
                    composite_score = ?,
                    tested_at = CURRENT_TIMESTAMP
                WHERE id = ?
                "#
            )
            .bind(result.total_return)
            .bind(result.sharpe_ratio)
            .bind(result.sortino_ratio)
            .bind(result.max_drawdown)
            .bind(result.win_rate)
            .bind(result.profit_factor)
            .bind(result.total_trades)
            .bind(result.annualized_return)
            .bind(result.max_drawdown_percent)
            .bind(result.total_profit)
            .bind(result.total_loss)
            .bind(result.max_consecutive_wins)
            .bind(result.max_consecutive_losses)
            .bind(result.trades_per_month)
            .bind(result.trades_per_year)
            .bind(result.stop_loss_exits)
            .bind(result.take_profit_exits)
            .bind(result.signal_exits)
            .bind(result.end_of_data_exits)
            .bind(result.composite_score)
            .bind(existing_result.id)
            .execute(&self.pool)
            .await?;

            Ok(existing_result.id.unwrap_or(0))
        } else {
            // Crear nuevo resultado
            self.create_extended(result).await
        }
    }

    /// Obtiene mejores resultados por composite_score
    pub async fn top_by_composite_score(&self, limit: i32) -> Result<Vec<BacktestResult>, sqlx::Error> {
        sqlx::query_as::<_, BacktestResult>(
            "SELECT * FROM backtest_results WHERE composite_score IS NOT NULL ORDER BY composite_score DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
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
            annualized_return: None,
            max_drawdown_percent: None,
            total_profit: None,
            total_loss: None,
            max_consecutive_wins: None,
            max_consecutive_losses: None,
            trades_per_month: None,
            trades_per_year: None,
            stop_loss_exits: None,
            take_profit_exits: None,
            signal_exits: None,
            end_of_data_exits: None,
            composite_score: None,
        };

        let id = repo.create(&result).await.unwrap();
        let results = repo.find_by_strategy(strategy_id).await.unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].sharpe_ratio, 2.3);
    }
}