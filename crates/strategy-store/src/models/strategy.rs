// ============================================================================
// crates/strategy-store/src/models/strategy.rs
// ============================================================================
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Strategy {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub source_code: String,
    pub format: String,
    pub parameters: Option<String>,
    pub sharpe_ratio: Option<f64>,
    pub total_return: Option<f64>,
    pub max_drawdown: Option<f64>,
    pub complexity_score: Option<f64>,
    pub created_at: Option<String>,
    // New fields for best strategies
    pub strategy_ast_json: Option<String>, // Full AST serialized as JSON
    pub is_best: Option<i32>, // 1 if best, 0 otherwise
    pub execution_metadata: Option<String>, // JSON with dataset, config, etc.
    pub strategy_hash: Option<String>, // Hash for deduplication
}

impl Strategy {
    pub fn new(name: String, source_code: String, format: String) -> Self {
        Self {
            id: None,
            name,
            description: None,
            source_code,
            format,
            parameters: None,
            sharpe_ratio: None,
            total_return: None,
            max_drawdown: None,
            complexity_score: None,
            created_at: None,
            strategy_ast_json: None,
            is_best: Some(0),
            execution_metadata: None,
            strategy_hash: None,
        }
    }
}