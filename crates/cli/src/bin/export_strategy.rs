//! Exporta una estrategia desde SQLite a un archivo Rust con el AST embebido.
//! Ejemplo:
//!   cargo run --bin export_strategy -- --id 123 --out-dir /tmp/exports

use clap::Parser;
use darwinx_core::TimeFrame;
use darwinx_generator::ast::nodes::StrategyAST;
use darwinx_store::{init_sqlite, StrategyRepository};
use serde_json::Value;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "export_strategy")]
#[command(about = "Exporta estrategia por id a un archivo .rs con el AST embebido")]
struct Config {
    /// Nombre de la estrategia (strategy_name en los resultados)
    #[arg(long)]
    strategy_name: String,

    /// Ruta a la base SQLite
    #[arg(long, default_value = "data/strategies.db")]
    db_path: String,

    /// Directorio de salida (default: cwd)
    #[arg(long)]
    out_dir: Option<String>,
}

fn timeframe_to_str(tf: TimeFrame) -> &'static str {
    match tf {
        TimeFrame::M1 => "1m",
        TimeFrame::M5 => "5m",
        TimeFrame::M15 => "15m",
        TimeFrame::M30 => "30m",
        TimeFrame::H1 => "1h",
        TimeFrame::H4 => "4h",
        TimeFrame::D1 => "1d",
        TimeFrame::W1 => "1w",
        TimeFrame::MN1 => "1mo",
    }
}

fn extract_pair(strategy_meta: Option<&str>) -> String {
    if let Some(meta_str) = strategy_meta {
        if let Ok(v) = serde_json::from_str::<Value>(meta_str) {
            if let Some(data_file) = v
                .get("data_file")
                .and_then(|d| d.as_str())
                .or_else(|| v.pointer("/config/data_file").and_then(|d| d.as_str()))
            {
                if let Some(base) = Path::new(data_file).file_name().and_then(|b| b.to_str()) {
                    let upper = base.to_ascii_uppercase();
                    let parts: Vec<&str> = upper.split('_').collect();
                    if !parts.is_empty() {
                        return parts[0].to_string();
                    }
                }
            }
        }
    }
    "PAIR".to_string()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::parse();

    let pool = init_sqlite(&config.db_path).await?;
    let repo = StrategyRepository::new(pool);

    let strategy = repo
        .find_by_name(&config.strategy_name)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Estrategia '{}' no encontrada", config.strategy_name))?;

    let ast_json = strategy
        .strategy_ast_json
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Estrategia '{}' no tiene AST almacenado", config.strategy_name))?;

    let ast: StrategyAST = darwinx_store::helpers::model_to_strategy_ast(&strategy)?;
    let ast_value: Value = serde_json::from_str(ast_json)?;
    let timeframe_str = timeframe_to_str(ast.timeframe);
    let pair = extract_pair(strategy.execution_metadata.as_deref());

    let out_dir = config
        .out_dir
        .as_deref()
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    if !out_dir.exists() {
        create_dir_all(&out_dir)?;
    }

    let safe_name = config
        .strategy_name
        .replace([' ', '/'], "_");
    let filename = format!("strategy_{}_{}_{}.rs", safe_name, timeframe_str, pair);
    let out_path = out_dir.join(filename);

    let mut file = File::create(&out_path)?;
    let pretty_ast = serde_json::to_string_pretty(&ast_value)?;

    // Generar módulo Rust con el AST embebido
    let content = format!(
        "// Autogenerado desde SQLite (nombre: {name})\n\
         // Pair: {pair}\n\
         // Timeframe: {tf}\n\
         // Ruta DB: {db}\n\
         use darwinx_generator::ast::nodes::StrategyAST;\n\
         use serde_json;\n\
\n\
         pub const STRATEGY_NAME: &str = \"{name}\";\n\
         pub const STRATEGY_JSON: &str = r#\"{json}\"#;\n\
\n\
         pub fn load_strategy_ast() -> StrategyAST {{\n\
             serde_json::from_str(STRATEGY_JSON).expect(\"AST válido\")\n\
         }}\n",
        name = config.strategy_name,
        pair = pair,
        tf = timeframe_str,
        db = config.db_path,
        json = pretty_ast.replace('\"', "\\\""),
    );

    file.write_all(content.as_bytes())?;

    println!(
        "✅ Estrategia '{}' exportada a {}",
        config.strategy_name,
        out_path.display()
    );

    Ok(())
}

