//! Helper functions for strategy store operations

use crate::models::Strategy;
use darwinx_generator::ast::nodes::StrategyAST;
use sha2::{Sha256, Digest};
use serde_json;

/// Calcula el hash SHA256 de una estrategia para deduplicación
pub fn calculate_strategy_hash(strategy_ast: &StrategyAST) -> String {
    let json = serde_json::to_string(strategy_ast)
        .expect("StrategyAST should be serializable");
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    let hash = hasher.finalize();
    hex::encode(hash)
}

/// Convierte StrategyAST a Strategy model para guardar en DB
pub fn strategy_ast_to_model(
    strategy_ast: &StrategyAST,
    metrics: Option<&darwinx_backtest_engine::BacktestMetrics>,
    execution_metadata: Option<serde_json::Value>,
) -> Strategy {
    let ast_json = serde_json::to_string(strategy_ast)
        .expect("StrategyAST should be serializable");
    let hash = calculate_strategy_hash(strategy_ast);
    
    let metadata_json = execution_metadata
        .map(|m| serde_json::to_string(&m).ok())
        .flatten();

    Strategy {
        id: None,
        name: strategy_ast.name.clone(),
        description: None,
        source_code: String::new(), // TODO: Convert AST to source code if needed
        format: "ast".to_string(),
        parameters: None,
        sharpe_ratio: metrics.map(|m| m.sharpe_ratio),
        total_return: metrics.map(|m| m.total_return),
        max_drawdown: metrics.map(|m| m.max_drawdown),
        complexity_score: Some(strategy_ast.complexity() as f64),
        created_at: None,
        strategy_ast_json: Some(ast_json),
        is_best: Some(1), // Mark as best when saving from massive backtest
        execution_metadata: metadata_json,
        strategy_hash: Some(hash),
    }
}

/// Convierte Strategy model a StrategyAST
pub fn model_to_strategy_ast(strategy: &Strategy) -> Result<StrategyAST, anyhow::Error> {
    if let Some(ref ast_json) = strategy.strategy_ast_json {
        let ast: StrategyAST = serde_json::from_str(ast_json)?;
        Ok(ast)
    } else {
        Err(anyhow::anyhow!("Strategy does not have AST JSON"))
    }
}

