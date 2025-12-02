//! Generación masiva, backtest y filtrado de mejores estrategias
//!
//! Ejecutar con: cargo run --bin massive_backtest -- --help
//!
//! Ejemplo:
//!   cargo run --bin massive_backtest -- \
//!     --strategies 10000 \
//!     --data data/BTCUSDT_1h.parquet \
//!     --top 100 \
//!     --min-trades 10 \
//!     --min-win-rate 0.4 \
//!     --min-sharpe 0.0

use clap::Parser;
use darwinx_generator::RandomGenerator;
use darwinx_data::{CsvLoader, ParquetLoader};
use darwinx_backtest_engine::{
    PolarsVectorizedBacktestEngine,
    BacktestConfig,
    BacktestResult,
};
use serde_json;
use std::collections::HashMap;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;
use chrono::NaiveDate;
use tokio;

/// Configuración para el pipeline de backtest masivo
#[derive(Parser, Debug)]
#[command(name = "massive_backtest")]
#[command(about = "Genera estrategias masivamente, las backtestea y filtra las mejores", long_about = None)]
struct Config {
    /// Número de estrategias a generar (10,000 - 100,000)
    #[arg(short, long, default_value_t = 10000)]
    strategies: usize,

    /// Ruta al archivo con datos históricos (CSV o Parquet)
    #[arg(short = 'd', long, default_value = "data/btcusdt_1h.csv")]
    data: String,

    /// Fecha de inicio del backtest (formato: YYYY-MM-DD)
    #[arg(long)]
    start_date: Option<String>,

    /// Fecha de fin del backtest (formato: YYYY-MM-DD)
    #[arg(long)]
    end_date: Option<String>,

    /// Número de mejores estrategias a seleccionar
    #[arg(short, long, default_value_t = 100)]
    top: usize,

    /// Balance inicial para el backtest
    #[arg(long, default_value_t = 10000.0)]
    initial_balance: f64,

    /// Comisión por trade (como porcentaje, ej: 0.001 = 0.1%)
    #[arg(long, default_value_t = 0.001)]
    commission_rate: f64,

    /// Slippage en basis points (ej: 5 = 0.05%)
    #[arg(long, default_value_t = 5.0)]
    slippage_bps: f64,

    /// Riesgo por trade como porcentaje del balance (ej: 0.02 = 2%)
    #[arg(long, default_value_t = 0.02)]
    risk_per_trade: f64,

    /// Filtros de calidad
    /// Mínimo número de trades requeridos
    #[arg(long, default_value_t = 10)]
    min_trades: usize,

    /// Mínimo win rate requerido (0.0 - 1.0)
    #[arg(long, default_value_t = 0.4)]
    min_win_rate: f64,

    /// Mínimo Sharpe ratio requerido
    #[arg(long, default_value_t = 0.0)]
    min_sharpe: f64,

    /// Mínimo retorno total requerido (como porcentaje, ej: 0.0 = 0%)
    #[arg(long, default_value_t = 0.0)]
    min_return: f64,

    /// Máximo drawdown permitido (como porcentaje, ej: 0.5 = 50%)
    #[arg(long, default_value_t = 0.5)]
    max_drawdown: f64,

    /// Pesos para el score compuesto (Sharpe, Sortino, Profit Factor, Return, Drawdown)
    #[arg(long, value_delimiter = ',', num_args = 5, default_values_t = vec![0.3, 0.2, 0.2, 0.15, 0.15])]
    score_weights: Vec<f64>,

    /// Guardar resultados en archivo JSON
    #[arg(short, long)]
    output: Option<String>,

    /// Mostrar top N estrategias en consola
    #[arg(long, default_value_t = 10)]
    show_top: usize,

    /// Modo verbose (mostrar más información)
    #[arg(short, long)]
    verbose: bool,
}

/// Parsea una fecha en formato YYYY-MM-DD a timestamp en milisegundos
fn parse_date(date_str: &str) -> anyhow::Result<i64> {
    let dt = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|e| anyhow::anyhow!("Formato de fecha inválido: {}. Use YYYY-MM-DD (ej: 2024-01-01)", e))?;
    let datetime = dt.and_hms_opt(0, 0, 0)
        .ok_or_else(|| anyhow::anyhow!("Fecha inválida"))?;
    Ok(datetime.and_utc().timestamp_millis())
}

/// Formatea un timestamp en milisegundos a string legible
fn format_timestamp(ts: i64) -> String {
    if let Some(dt) = chrono::DateTime::from_timestamp_millis(ts) {
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        format!("{}", ts)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::parse();

    if config.verbose {
        println!("🚀 DarwinX - Generación Masiva y Backtest");
        println!("==========================================\n");
        println!("📋 Configuración:");
        println!("   Estrategias a generar: {}", config.strategies);
        println!("   Archivo de datos:     {}", config.data);
        if let Some(start) = &config.start_date {
            println!("   Fecha inicio:        {}", start);
        }
        if let Some(end) = &config.end_date {
            println!("   Fecha fin:           {}", end);
        }
        println!("   Top N a seleccionar:  {}", config.top);
        println!("   Balance inicial:      ${:.2}", config.initial_balance);
        println!("   Comisión:            {:.4}%", config.commission_rate * 100.0);
        println!("   Slippage:            {:.2} bps", config.slippage_bps);
        println!("   Riesgo por trade:    {:.2}%", config.risk_per_trade * 100.0);
        println!();
    }

    // ==========================================
    // FASE 1: Generación Masiva de Estrategias
    // ==========================================
    if config.verbose {
        println!("📝 FASE 1: Generando estrategias masivamente...");
    }
    let generator = RandomGenerator::new();
    let strategies = generator.generate_batch(config.strategies);
    
    if config.verbose {
        println!("   ✅ Generadas {} estrategias\n", strategies.len());
    }

    // ==========================================
    // FASE 2: Cargar Datos Históricos
    // ==========================================
    if config.verbose {
        println!("📊 FASE 2: Cargando datos históricos...");
    }
    
    // Detectar formato por extensión del archivo
    let data_path = Path::new(&config.data);
    let extension = data_path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    let mut candles = match extension.as_str() {
        "parquet" => {
            if config.verbose {
                println!("   📦 Detectado formato Parquet");
            }
            match ParquetLoader::load(&config.data) {
                Ok(candles) => {
                    if config.verbose {
                        println!("   ✅ Cargadas {} velas desde {}", candles.len(), config.data);
                    }
                    candles
                }
                Err(e) => {
                    eprintln!("   ❌ Error al cargar archivo Parquet: {}", e);
                    eprintln!("   💡 Asegúrate de que el archivo existe y tiene el formato correcto:");
                    eprintln!("      Columnas: timestamp, open, high, low, close, volume");
                    return Err(e);
                }
            }
        }
        "csv" => {
            if config.verbose {
                println!("   📄 Detectado formato CSV");
            }
            match CsvLoader::load(&config.data) {
                Ok(candles) => {
                    if config.verbose {
                        println!("   ✅ Cargadas {} velas desde {}", candles.len(), config.data);
                    }
                    candles
                }
                Err(e) => {
                    eprintln!("   ❌ Error al cargar archivo CSV: {}", e);
                    eprintln!("   💡 Asegúrate de que el archivo existe y tiene el formato correcto:");
                    eprintln!("      timestamp,open,high,low,close,volume");
                    return Err(e);
                }
            }
        }
        _ => {
            eprintln!("   ❌ Formato de archivo no soportado: {}", extension);
            eprintln!("   💡 Formatos soportados: .csv, .parquet");
            eprintln!("   💡 Archivo especificado: {}", config.data);
            return Err(anyhow::anyhow!("Formato de archivo no soportado: {}", extension));
        }
    };
    
    // Filtrar por fecha si se especificó
    if let Some(start) = &config.start_date {
        let start_ts = parse_date(start)?;
        let before = candles.len();
        candles.retain(|c| c.timestamp >= start_ts);
        if config.verbose {
            println!("   📅 Filtrado por fecha inicio ({}): {} velas eliminadas", 
                start, before - candles.len());
        }
    }

    if let Some(end) = &config.end_date {
        let end_ts = parse_date(end)?;
        let before = candles.len();
        candles.retain(|c| c.timestamp <= end_ts);
        if config.verbose {
            println!("   📅 Filtrado por fecha fin ({}): {} velas eliminadas", 
                end, before - candles.len());
        }
    }

    // Validar que haya velas después del filtrado
    if candles.is_empty() {
        return Err(anyhow::anyhow!(
            "No hay velas después del filtrado por fecha. Verifica las fechas especificadas."
        ));
    }

    if config.verbose {
        if config.start_date.is_some() || config.end_date.is_some() {
            let start_ts = candles[0].timestamp;
            let end_ts = candles[candles.len() - 1].timestamp;
            println!("   📅 Período del backtest: {} - {}", 
                format_timestamp(start_ts), 
                format_timestamp(end_ts));
        }
        println!();
    }

    // ==========================================
    // FASE 3: Configurar Backtest
    // ==========================================
    if config.verbose {
        println!("⚙️  FASE 3: Configurando backtest...");
    }
    let backtest_config = BacktestConfig {
        initial_balance: config.initial_balance,
        commission_rate: config.commission_rate,
        slippage_bps: config.slippage_bps,
        max_positions: 1,
        risk_per_trade: config.risk_per_trade,
    };
    if config.verbose {
        println!("   ✅ Configuración lista\n");
    }

    // ==========================================
    // FASE 4: Backtest Masivo con Polars
    // ==========================================
    if config.verbose {
        println!("🔥 FASE 4: Ejecutando backtest masivo con Polars...");
        println!("   (Esto puede tardar varios minutos para {} estrategias)", config.strategies);
    }
    
    let engine = PolarsVectorizedBacktestEngine::new();
    
    // Crear un mapa de nombre de estrategia -> AST para guardar las definiciones completas
    use std::collections::HashMap;
    let strategies_map: HashMap<String, darwinx_generator::StrategyAST> = strategies
        .iter()
        .map(|s| (s.name.clone(), s.clone()))
        .collect();
    
    let start_time = std::time::Instant::now();
    let results = match engine.run_massive_backtest(strategies, candles, &backtest_config).await {
        Ok(results) => {
            let elapsed = start_time.elapsed();
            if config.verbose {
                println!("   ✅ Backtest completado en {:.2} segundos", elapsed.as_secs_f64());
                println!("   ✅ Resultados: {} estrategias backtesteadas\n", results.len());
            } else {
                println!("✅ Backtest completado: {} estrategias en {:.2}s", results.len(), elapsed.as_secs_f64());
            }
            results
        }
        Err(e) => {
            eprintln!("   ❌ Error en backtest: {}", e);
            return Err(e.into());
        }
    };

    // ==========================================
    // FASE 5: Filtrar y Rankear Mejores
    // ==========================================
    if config.verbose {
        println!("🏆 FASE 5: Filtrando y rankeando mejores estrategias...");
    }
    
    // Filtrar estrategias con métricas mínimas
    let filtered: Vec<&BacktestResult> = results
        .iter()
        .filter(|r| {
            let m = &r.metrics;
            m.total_trades >= config.min_trades &&
            m.win_rate >= config.min_win_rate &&
            m.sharpe_ratio >= config.min_sharpe &&
            m.total_return >= config.min_return &&
            m.max_drawdown <= config.max_drawdown
        })
        .collect();
    
    if config.verbose {
        println!("   📊 Estrategias que pasan filtros: {}/{}", filtered.len(), results.len());
    }

    // Validar y normalizar pesos del score
    let weights = if config.score_weights.len() == 5 {
        let sum: f64 = config.score_weights.iter().sum();
        if sum > 0.0 {
            config.score_weights.iter().map(|w| w / sum).collect::<Vec<f64>>()
        } else {
            vec![0.3, 0.2, 0.2, 0.15, 0.15] // Default
        }
    } else {
        vec![0.3, 0.2, 0.2, 0.15, 0.15] // Default
    };

    // Rankear por score compuesto
    let mut ranked: Vec<(f64, &BacktestResult)> = filtered
        .iter()
        .map(|r| {
            let m = &r.metrics;
            
            // Normalizar métricas (0-1)
            let sharpe_norm = (m.sharpe_ratio / 5.0).max(0.0).min(1.0); // Sharpe típico 0-5
            let sortino_norm = (m.sortino_ratio / 5.0).max(0.0).min(1.0);
            let pf_norm = (m.profit_factor / 5.0).max(0.0).min(1.0); // PF típico 0-5
            let return_norm = (m.total_return * 2.0).max(0.0).min(1.0); // Return 0-50%
            let dd_norm = 1.0 - (m.max_drawdown * 2.0).max(0.0).min(1.0); // Drawdown inverso
            
            // Score ponderado
            let score = 
                weights[0] * sharpe_norm +
                weights[1] * sortino_norm +
                weights[2] * pf_norm +
                weights[3] * return_norm +
                weights[4] * dd_norm;
            
            (score, *r)
        })
        .collect();
    
    // Ordenar por score descendente
    ranked.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    
    // Seleccionar top N
    let top_strategies: Vec<&BacktestResult> = ranked
        .iter()
        .take(config.top)
        .map(|(_, r)| *r)
        .collect();
    
    if config.verbose {
        println!("   ✅ Top {} estrategias seleccionadas\n", top_strategies.len());
    }

    // ==========================================
    // FASE 6: Mostrar Resultados
    // ==========================================
    if config.verbose {
        println!("📈 FASE 6: Top {} Estrategias", config.show_top);
        println!("{}", "=".repeat(100));
    }
    
    for (i, result) in top_strategies.iter().take(config.show_top).enumerate() {
        let m = &result.metrics;
        if config.verbose {
            println!("\n{}. {}", i + 1, result.strategy_name);
            println!("   📊 Métricas:");
            println!("      Total Return:     {:.2}%", m.total_return * 100.0);
            println!("      Sharpe Ratio:     {:.3}", m.sharpe_ratio);
            println!("      Sortino Ratio:    {:.3}", m.sortino_ratio);
            println!("      Profit Factor:   {:.3}", m.profit_factor);
            println!("      Max Drawdown:     {:.2}%", m.max_drawdown_percent * 100.0);
            println!("      Win Rate:         {:.2}%", m.win_rate * 100.0);
            println!("      Total Trades:     {}", m.total_trades);
            println!("      Avg Win:          ${:.2}", m.average_win);
            println!("      Avg Loss:         ${:.2}", m.average_loss);
            println!("      Total Profit:     ${:.2}", m.total_profit);
            println!("      Total Loss:       ${:.2}", m.total_loss);
            println!("      Avg Trade Duration: {:.1} horas", m.average_trade_duration_ms / (1000.0 * 60.0 * 60.0));
            println!("      Avg Win Duration:   {:.1} horas", m.average_winning_trade_duration_ms / (1000.0 * 60.0 * 60.0));
            println!("      Avg Loss Duration:  {:.1} horas", m.average_losing_trade_duration_ms / (1000.0 * 60.0 * 60.0));
            println!("      Max Consecutive Wins: {}", m.max_consecutive_wins);
            println!("      Max Consecutive Losses: {}", m.max_consecutive_losses);
            println!("      Trades/Month:     {:.1}", m.trades_per_month);
            println!("      Trades/Year:      {:.1}", m.trades_per_year);
        } else {
            println!("{}. {} | Return: {:.2}% | Sharpe: {:.3} | Trades: {} | Win Rate: {:.2}% | DD: {:.2}%",
                i + 1, result.strategy_name, 
                m.total_return * 100.0, m.sharpe_ratio, m.total_trades, m.win_rate * 100.0, m.max_drawdown_percent * 100.0);
        }
    }
    
    if config.verbose {
        println!("\n{}", "=".repeat(100));
    }
    
    println!("\n📊 Resumen Final:");
    println!("   Total generadas:        {}", config.strategies);
    println!("   Backtesteadas:         {}", results.len());
    println!("   Pasaron filtros:       {}", filtered.len());
    println!("   Top {} seleccionadas: {}", config.top, top_strategies.len());

    // ==========================================
    // FASE 7: Guardar Resultados (opcional)
    // ==========================================
    if let Some(output_path) = &config.output {
        if config.verbose {
            println!("\n💾 Guardando resultados en {}...", output_path);
        }
        
        // Crear estructura serializable con resultados y scores
        let output_data = serde_json::json!({
            "config": {
                "strategies_generated": config.strategies,
                "data_file": config.data,
                "top_n": config.top,
                "filters": {
                    "min_trades": config.min_trades,
                    "min_win_rate": config.min_win_rate,
                    "min_sharpe": config.min_sharpe,
                    "min_return": config.min_return,
                    "max_drawdown": config.max_drawdown,
                },
                "score_weights": weights,
            },
            "summary": {
                "total_backtested": results.len(),
                "passed_filters": filtered.len(),
                "top_selected": top_strategies.len(),
            },
            "top_strategies": top_strategies.iter().enumerate().map(|(i, r)| {
                let score = ranked[i].0;
                // Obtener la definición completa de la estrategia (AST)
                let strategy_ast = strategies_map.get(&r.strategy_name);
                serde_json::json!({
                    "rank": i + 1,
                    "score": score,
                    "strategy_name": r.strategy_name,
                    "strategy": strategy_ast, // AST completo de la estrategia
                    "metrics": r.metrics,
                    "total_trades": r.trades.len(),
                })
            }).collect::<Vec<_>>(),
        });
        
        // Crear directorio si no existe
        if let Some(parent) = Path::new(output_path).parent() {
            create_dir_all(parent)?;
        }
        
        let json = serde_json::to_string_pretty(&output_data)?;
        let mut file = File::create(output_path)?;
        file.write_all(json.as_bytes())?;
        
        if config.verbose {
            println!("   ✅ Resultados guardados en {}", output_path);
        } else {
            println!("💾 Resultados guardados en {}", output_path);
        }
    }

    Ok(())
}

