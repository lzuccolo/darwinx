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

    /// Crea o actualiza una estrategia con AST completo (para mejores estrategias)
    pub async fn create_or_update_best(
        &self,
        strategy: &Strategy,
    ) -> Result<i64, sqlx::Error> {
        // Primero intentar encontrar por hash (deduplicación)
        if let Some(ref hash) = strategy.strategy_hash {
            if let Some(existing) = self.find_by_hash(hash).await? {
                // Actualizar métricas si la nueva es mejor
                if let (Some(new_sharpe), Some(existing_sharpe)) = 
                    (strategy.sharpe_ratio, existing.sharpe_ratio) {
                    if new_sharpe > existing_sharpe {
                        self.update_metrics(
                            existing.id.unwrap(),
                            new_sharpe,
                            strategy.total_return.unwrap_or(0.0),
                            strategy.max_drawdown.unwrap_or(0.0),
                        ).await?;
                    }
                }
                return Ok(existing.id.unwrap());
            }
        }

        // Crear nueva estrategia
        let result = sqlx::query(
            r#"
            INSERT INTO strategies 
            (name, description, source_code, format, parameters, complexity_score,
             sharpe_ratio, total_return, max_drawdown, strategy_ast_json, is_best,
             execution_metadata, strategy_hash)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&strategy.name)
        .bind(&strategy.description)
        .bind(&strategy.source_code)
        .bind(&strategy.format)
        .bind(&strategy.parameters)
        .bind(strategy.complexity_score)
        .bind(strategy.sharpe_ratio)
        .bind(strategy.total_return)
        .bind(strategy.max_drawdown)
        .bind(&strategy.strategy_ast_json)
        .bind(strategy.is_best.unwrap_or(0))
        .bind(&strategy.execution_metadata)
        .bind(&strategy.strategy_hash)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Busca estrategia por hash (para deduplicación)
    pub async fn find_by_hash(&self, hash: &str) -> Result<Option<Strategy>, sqlx::Error> {
        sqlx::query_as::<_, Strategy>("SELECT * FROM strategies WHERE strategy_hash = ?")
            .bind(hash)
            .fetch_optional(&self.pool)
            .await
    }

    /// Obtiene las mejores estrategias (is_best = 1) ordenadas por Sharpe
    pub async fn get_best_strategies(&self, limit: i32) -> Result<Vec<Strategy>, sqlx::Error> {
        sqlx::query_as::<_, Strategy>(
            "SELECT * FROM strategies WHERE is_best = 1 ORDER BY sharpe_ratio DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    /// Marca estrategias como mejores (is_best = 1)
    pub async fn mark_as_best(&self, ids: &[i64]) -> Result<(), sqlx::Error> {
        // Primero desmarcar todas
        sqlx::query("UPDATE strategies SET is_best = 0")
            .execute(&self.pool)
            .await?;
        
        // Marcar las nuevas como mejores
        for id in ids {
            sqlx::query("UPDATE strategies SET is_best = 1 WHERE id = ?")
                .bind(id)
                .execute(&self.pool)
                .await?;
        }
        
        Ok(())
    }

    /// Obtiene las mejores estrategias como StrategyAST (para retroalimentación genética)
    pub async fn get_best_strategies_as_ast(
        &self,
        limit: i32,
    ) -> Result<Vec<darwinx_generator::ast::nodes::StrategyAST>, anyhow::Error> {
        use crate::helpers::model_to_strategy_ast;
        
        let strategies = self.get_best_strategies(limit).await?;
        let mut asts = Vec::new();
        
        for strategy in strategies {
            match model_to_strategy_ast(&strategy) {
                Ok(ast) => asts.push(ast),
                Err(e) => {
                    eprintln!("Warning: No se pudo convertir estrategia {} a AST: {}", strategy.name, e);
                }
            }
        }
        
        Ok(asts)
    }

    /// Obtiene todas las estrategias con AST (no solo las mejores) ordenadas por Sharpe
    pub async fn get_strategies_with_ast(
        &self,
        limit: i32,
    ) -> Result<Vec<darwinx_generator::ast::nodes::StrategyAST>, anyhow::Error> {
        use crate::helpers::model_to_strategy_ast;
        
        let strategies = sqlx::query_as::<_, Strategy>(
            "SELECT * FROM strategies WHERE strategy_ast_json IS NOT NULL ORDER BY sharpe_ratio DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        
        let mut asts = Vec::new();
        
        for strategy in strategies {
            match model_to_strategy_ast(&strategy) {
                Ok(ast) => asts.push(ast),
                Err(e) => {
                    eprintln!("Warning: No se pudo convertir estrategia {} a AST: {}", strategy.name, e);
                }
            }
        }
        
        Ok(asts)
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