//! Repositorio de estrategias

use crate::models::Strategy;
use sqlx::{Pool, Sqlite};

pub struct StrategyRepository {
    pool: Pool<Sqlite>,
}

impl StrategyRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// Crea una nueva estrategia
    pub async fn create(&self, strategy: &Strategy) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO strategies (name, description, source_code, format, parameters, complexity_score)
            VALUES (?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&strategy.name)
        .bind(&strategy.description)
        .bind(&strategy.source_code)
        .bind(&strategy.format)
        .bind(&strategy.parameters)
        .bind(strategy.complexity_score)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Busca una estrategia por ID
    pub async fn find_by_id(&self, id: i64) -> Result<Option<Strategy>, sqlx::Error> {
        sqlx::query_as::<_, Strategy>("SELECT * FROM strategies WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    /// Lista estrategias con paginación
    pub async fn list(&self, page: i32, page_size: i32) -> Result<Vec<Strategy>, sqlx::Error> {
        let offset = (page - 1) * page_size;
        sqlx::query_as::<_, Strategy>(
            "SELECT * FROM strategies ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Elimina una estrategia
    pub async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM strategies WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    /// Actualiza las métricas de una estrategia
    pub async fn update_metrics(
        &self,
        id: i64,
        sharpe: f64,
        total_return: f64,
        max_dd: f64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE strategies SET sharpe_ratio = ?, total_return = ?, max_drawdown = ? WHERE id = ?"
        )
        .bind(sharpe)
        .bind(total_return)
        .bind(max_dd)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Cuenta el total de estrategias
    pub async fn count(&self) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM strategies")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_and_find() {
        let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        let repo = StrategyRepository::new(pool);
        let strategy = Strategy::new(
            "Test Strategy".to_string(),
            "fn main() {}".to_string(),
            "rust".to_string(),
        );

        let id = repo.create(&strategy).await.unwrap();
        let found = repo.find_by_id(id).await.unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test Strategy");
    }
}