//! # DarwinX Strategy Store
//!
//! Sistema de persistencia para estrategias y resultados de backtest

pub mod models;
pub mod repositories;
pub mod database;
pub mod helpers;

// Re-exports
pub use models::{BacktestResult, Strategy, Trade};
pub use repositories::{BacktestRepository, StrategyRepository};
pub use database::init_sqlite;
pub use helpers::{calculate_strategy_hash, strategy_ast_to_model, model_to_strategy_ast};

/// Carga mejores estrategias desde SQLite para usar como población inicial en genética
pub async fn load_best_strategies_for_genetics(
    db_path: &str,
    count: usize,
) -> Result<Vec<darwinx_generator::ast::nodes::StrategyAST>, anyhow::Error> {
    let pool = init_sqlite(db_path).await?;
    let repo = StrategyRepository::new(pool);
    
    // Intentar cargar mejores estrategias primero
    let mut strategies = repo.get_best_strategies_as_ast(count as i32).await?;
    
    // Si no hay suficientes mejores, completar con otras estrategias
    if strategies.len() < count {
        let additional = repo.get_strategies_with_ast((count - strategies.len()) as i32).await?;
        strategies.extend(additional);
    }
    
    Ok(strategies)
}