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
use darwinx_generator::{RandomGenerator, GeneticGenerator, GeneticConfig};
use darwinx_data::{CsvLoader, ParquetLoader};
use darwinx_backtest_engine::{
    PolarsVectorizedBacktestEngine,
    BacktestConfig,
    BacktestResult,
};
use darwinx_store::{
    init_sqlite,
    StrategyRepository,
    BacktestRepository,
    strategy_ast_to_model,
    load_best_strategies_for_genetics,
};
use serde_json;
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

    /// Stop loss como porcentaje del precio de entrada (ej: 0.02 = 2%, 0 = deshabilitado)
    #[arg(long)]
    stop_loss: Option<f64>,

    /// Take profit como porcentaje del precio de entrada (ej: 0.05 = 5%, 0 = deshabilitado)
    #[arg(long)]
    take_profit: Option<f64>,

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

    /// Guardar resultados en archivo JSON (opcional, solo para consulta rápida)
    #[arg(short, long)]
    output: Option<String>,

    /// Ruta a la base de datos SQLite para persistir mejores estrategias (default: data/strategies.db)
    #[arg(long, default_value = "data/strategies.db")]
    db_path: String,

    /// No guardar en SQLite (solo JSON si se especifica --output)
    #[arg(long)]
    no_db: bool,

    /// Cargar mejores estrategias desde SQLite para usar como población inicial (número de estrategias a cargar)
    #[arg(long)]
    load_best: Option<usize>,

    /// Habilitar evolución genética después del backtest inicial (número de generaciones)
    #[arg(long)]
    evolve: Option<usize>,

    /// Tamaño de población para evolución genética (default: 100)
    #[arg(long, default_value_t = 100)]
    evolve_population: usize,

    /// Tasa de mutación para evolución genética (default: 0.1)
    #[arg(long, default_value_t = 0.1)]
    evolve_mutation_rate: f64,

    /// Tamaño de elite para evolución genética (default: 10)
    #[arg(long, default_value_t = 10)]
    evolve_elite_size: usize,

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
        if let Some(load_count) = config.load_best {
            println!("   Cargar desde DB:     {} estrategias", load_count);
        }
        println!("   Top N a seleccionar:  {}", config.top);
        println!("   Balance inicial:      ${:.2}", config.initial_balance);
        println!("   Comisión:            {:.4}%", config.commission_rate * 100.0);
        println!("   Slippage:            {:.2} bps", config.slippage_bps);
        println!("   Riesgo por trade:    {:.2}%", config.risk_per_trade * 100.0);
        if let Some(sl) = config.stop_loss {
            println!("   Stop Loss:           {:.2}%", sl * 100.0);
        }
        if let Some(tp) = config.take_profit {
            println!("   Take Profit:         {:.2}%", tp * 100.0);
        }
        println!();
    }

    // ==========================================
    // FASE 1: Generación Masiva de Estrategias
    // ==========================================
    if config.verbose {
        println!("📝 FASE 1: Generando estrategias masivamente...");
    }
    
    let generator = RandomGenerator::new();
    let mut strategies = Vec::new();
    
    // Cargar mejores estrategias desde SQLite si se especifica
    if let Some(load_count) = config.load_best {
        if config.verbose {
            println!("   🔄 Cargando {} mejores estrategias desde SQLite...", load_count);
        }
        
        match load_best_strategies_for_genetics(&config.db_path, load_count).await {
            Ok(loaded_strategies) => {
                strategies.extend(loaded_strategies);
                if config.verbose {
                    println!("   ✅ Cargadas {} estrategias desde SQLite", strategies.len());
                }
            }
            Err(e) => {
                eprintln!("   ⚠️  Error al cargar estrategias desde SQLite: {}", e);
                eprintln!("   💡 Continuando con generación aleatoria completa...");
            }
        }
    }
    
    // Completar con estrategias aleatorias si es necesario
    let remaining = config.strategies.saturating_sub(strategies.len());
    if remaining > 0 {
        if config.verbose {
            if !strategies.is_empty() {
                println!("   🎲 Generando {} estrategias aleatorias adicionales...", remaining);
            } else {
                println!("   🎲 Generando {} estrategias aleatorias...", remaining);
            }
        }
        let random_strategies = generator.generate_batch(remaining);
        strategies.extend(random_strategies);
    }
    
    if config.verbose {
        println!("   ✅ Total {} estrategias preparadas ({} desde DB, {} aleatorias)\n", 
            strategies.len(),
            config.load_best.unwrap_or(0),
            strategies.len() - config.load_best.unwrap_or(0));
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
    let backtest_config = BacktestConfig::with_risk_management(
        config.initial_balance,
        config.commission_rate,
        config.slippage_bps,
        1, // max_positions
        config.risk_per_trade,
        config.stop_loss,
        config.take_profit,
    );
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
    
    // Clonar candles para poder usarlo después si hay evolución
    let candles_for_backtest = candles.clone();
    let start_time = std::time::Instant::now();
    let results = match engine.run_massive_backtest(strategies, candles_for_backtest, &backtest_config).await {
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
    
    // DIAGNÓSTICO: Analizar estadísticas de métricas antes de filtrar
    if config.verbose && !results.is_empty() {
        let total_strategies = results.len();
        let strategies_with_trades: Vec<&BacktestResult> = results.iter()
            .filter(|r| r.metrics.total_trades > 0)
            .collect();
        
        if !strategies_with_trades.is_empty() {
            let mut total_trades_stats = Vec::new();
            let mut win_rate_stats = Vec::new();
            let mut return_stats = Vec::new();
            let mut drawdown_stats = Vec::new();
            let mut sharpe_stats = Vec::new();
            
            for r in &strategies_with_trades {
                total_trades_stats.push(r.metrics.total_trades as f64);
                win_rate_stats.push(r.metrics.win_rate);
                return_stats.push(r.metrics.total_return);
                drawdown_stats.push(r.metrics.max_drawdown);
                sharpe_stats.push(r.metrics.sharpe_ratio);
            }
            
            fn stats(vals: &[f64]) -> (f64, f64, f64, f64) {
                if vals.is_empty() { return (0.0, 0.0, 0.0, 0.0); }
                let min = vals.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                let max = vals.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                let sum: f64 = vals.iter().sum();
                let avg = sum / vals.len() as f64;
                (min, max, avg, sum)
            }
            
            let (trades_min, trades_max, trades_avg, _) = stats(&total_trades_stats);
            let (wr_min, wr_max, wr_avg, _) = stats(&win_rate_stats);
            let (ret_min, ret_max, ret_avg, _) = stats(&return_stats);
            let (dd_min, dd_max, dd_avg, _) = stats(&drawdown_stats);
            let (sharpe_min, sharpe_max, sharpe_avg, _) = stats(&sharpe_stats);
            
            println!("   📊 Diagnóstico de métricas ({} estrategias con trades de {} total):", strategies_with_trades.len(), total_strategies);
            println!("      Total Trades:  min={:.1}, max={:.1}, avg={:.1} (requiere >= {})", 
                trades_min, trades_max, trades_avg, config.min_trades);
            println!("      Win Rate:      min={:.3}, max={:.3}, avg={:.3} (requiere >= {:.3})", 
                wr_min, wr_max, wr_avg, config.min_win_rate);
            println!("      Total Return:  min={:.4}, max={:.4}, avg={:.4} (requiere >= {:.4})", 
                ret_min, ret_max, ret_avg, config.min_return);
            println!("      Max Drawdown:  min={:.4}, max={:.4}, avg={:.4} (requiere <= {:.4})", 
                dd_min, dd_max, dd_avg, config.max_drawdown);
            println!("      Sharpe Ratio:  min={:.3}, max={:.3}, avg={:.3} (requiere >= {:.3})", 
                sharpe_min, sharpe_max, sharpe_avg, config.min_sharpe);
            
            // Contar cuántas fallan cada filtro
            let fail_trades = results.iter().filter(|r| r.metrics.total_trades < config.min_trades).count();
            let fail_winrate = results.iter().filter(|r| r.metrics.win_rate < config.min_win_rate).count();
            let fail_return = results.iter().filter(|r| r.metrics.total_return < config.min_return).count();
            let fail_drawdown = results.iter().filter(|r| r.metrics.max_drawdown > config.max_drawdown).count();
            let fail_sharpe = results.iter().filter(|r| r.metrics.sharpe_ratio < config.min_sharpe).count();
            
            println!("   🔍 Estrategias que fallan cada filtro:");
            println!("      Faltan trades ({}):        {} estrategias", config.min_trades, fail_trades);
            println!("      Win rate bajo ({}):        {} estrategias", config.min_win_rate, fail_winrate);
            println!("      Return bajo ({}):          {} estrategias", config.min_return, fail_return);
            println!("      Drawdown alto ({}):        {} estrategias", config.max_drawdown, fail_drawdown);
            println!("      Sharpe bajo ({}):          {} estrategias", config.min_sharpe, fail_sharpe);
        } else {
            println!("   ⚠️  Ninguna estrategia generó trades!");
            
            // DIAGNÓSTICO ADICIONAL: Analizar algunas estrategias para ver por qué no generan trades
            println!("   🔍 Analizando primeras 5 estrategias sin trades...");
            let sample_strategies: Vec<(&BacktestResult, Option<&darwinx_generator::StrategyAST>)> = results.iter()
                .take(5)
                .map(|r| (r, strategies_map.get(&r.strategy_name)))
                .collect();
            
            for (i, (result, strategy_ast)) in sample_strategies.iter().enumerate() {
                println!("      Estrategia {}: {}", i + 1, result.strategy_name);
                println!("         - Trades generados: {}", result.metrics.total_trades);
                
                if let Some(ast) = strategy_ast {
                    println!("         - Entry rules: {} condiciones, operador: {:?}", 
                        ast.entry_rules.conditions.len(), 
                        ast.entry_rules.operator);
                    if !ast.entry_rules.conditions.is_empty() {
                        println!("         - Primera condición de entrada:");
                        let first_cond = &ast.entry_rules.conditions[0];
                        println!("           {} {:?} {:?}", 
                            first_cond.indicator.name,
                            first_cond.comparison,
                            first_cond.value);
                    }
                    println!("         - Exit rules: {} condiciones, operador: {:?}", 
                        ast.exit_rules.conditions.len(), 
                        ast.exit_rules.operator);
                } else {
                    println!("         - ⚠️  AST no disponible para esta estrategia");
                }
            }
            
            // Contar estrategias con condiciones vacías
            let empty_entry_rules = results.iter()
                .filter(|r| {
                    strategies_map.get(&r.strategy_name)
                        .map(|ast| ast.entry_rules.conditions.is_empty())
                        .unwrap_or(true)
                })
                .count();
            
            if empty_entry_rules > 0 {
                println!("   ⚠️  {} estrategias tienen condiciones de entrada vacías!", empty_entry_rules);
            }
            
            // Analizar tipos de condiciones más comunes
            let mut condition_types: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
            for result in results.iter().take(100) { // Muestra de 100 estrategias
                if let Some(ast) = strategies_map.get(&result.strategy_name) {
                    for cond in &ast.entry_rules.conditions {
                        let key = format!("{:?}", cond.comparison);
                        *condition_types.entry(key).or_insert(0) += 1;
                    }
                }
            }
            
            if !condition_types.is_empty() {
                println!("   📊 Tipos de comparaciones en muestra de 100 estrategias:");
                let mut sorted: Vec<_> = condition_types.iter().collect();
                sorted.sort_by(|a, b| b.1.cmp(a.1));
                for (comp_type, count) in sorted.iter().take(5) {
                    println!("      {}: {} ocurrencias", comp_type, count);
                }
            }
            
            println!("   💡 Posibles causas:");
            println!("      - Las condiciones pueden ser demasiado específicas (ej: Equals con valores exactos)");
            println!("      - Las condiciones con operador 'And' requieren que TODAS se cumplan simultáneamente");
            println!("      - Los indicadores pueden no estar calculándose correctamente");
            println!("      - Las condiciones 'CrossesAbove/Below' están simplificadas a comparaciones directas");
        }
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
    // FASE 6: Evolución Genética (opcional)
    // ==========================================
    let mut all_results = results.clone();
    let mut all_strategies_map = strategies_map.clone();
    
    // Clonar datos necesarios antes del bloque de evolución
    let top_strategy_names: Vec<String> = top_strategies.iter().map(|r| r.strategy_name.clone()).collect();
    let top_strategies_metrics: Vec<(String, darwinx_backtest_engine::BacktestMetrics)> = top_strategies
        .iter()
        .map(|r| (r.strategy_name.clone(), r.metrics.clone()))
        .collect();
    
    if let Some(generations) = config.evolve {
        if config.verbose {
            println!("🧬 FASE 6: Evolución Genética ({} generaciones)...", generations);
        }

        // Crear mapa de estrategia -> métricas para función de fitness
        let fitness_map: HashMap<String, darwinx_backtest_engine::BacktestMetrics> = top_strategies_metrics
            .into_iter()
            .collect();

        // Función de fitness basada en métricas de backtest
        let weights = config.score_weights.clone();
        let fitness_map_clone = fitness_map.clone();
        let fitness_fn = move |strategy: &darwinx_generator::StrategyAST| -> f64 {
            if let Some(metrics) = fitness_map_clone.get(&strategy.name) {
                // Normalizar métricas (similar al scoring)
                let sharpe_norm = (metrics.sharpe_ratio + 2.0) / 4.0; // -2 a 2 -> 0 a 1
                let sortino_norm = (metrics.sortino_ratio + 2.0) / 4.0;
                let pf_norm = (metrics.profit_factor.min(5.0)) / 5.0; // 0 a 5 -> 0 a 1
                let return_norm = (metrics.total_return + 1.0) / 2.0; // -1 a 1 -> 0 a 1
                let dd_norm = 1.0 - metrics.max_drawdown_percent.min(1.0); // Invertir (menor es mejor)

                // Score compuesto
                weights[0] * sharpe_norm +
                weights[1] * sortino_norm +
                weights[2] * pf_norm +
                weights[3] * return_norm +
                weights[4] * dd_norm
            } else {
                // Si no tiene métricas, usar complejidad como proxy
                -(strategy.complexity() as f64) * 0.1 // Penalizar complejidad
            }
        };

        // Obtener ASTs de las top estrategias usando nombres clonados
        let top_asts: Vec<darwinx_generator::StrategyAST> = top_strategy_names
            .iter()
            .filter_map(|name| strategies_map.get(name).cloned())
            .collect();

        if top_asts.is_empty() {
            if config.verbose {
                println!("   ⚠️  No hay estrategias para evolucionar\n");
            }
        } else {
            // Configurar generador genético
            let genetic_config = GeneticConfig {
                population_size: config.evolve_population,
                generations,
                mutation_rate: config.evolve_mutation_rate,
                elite_size: config.evolve_elite_size,
                tournament_size: 3,
            };
            let genetic_gen = GeneticGenerator::new(genetic_config);

            if config.verbose {
                println!("   🧬 Población inicial: {} estrategias", top_asts.len());
                println!("   🧬 Tamaño de población: {}", config.evolve_population);
                println!("   🧬 Generaciones: {}", generations);
            }

            // Evolucionar
            let evolved_strategies = genetic_gen.evolve(top_asts, fitness_fn);

            if config.verbose {
                println!("   ✅ Evolución completada: {} estrategias evolucionadas", evolved_strategies.len());
            }

            // Agregar estrategias evolucionadas al mapa
            for strategy in &evolved_strategies {
                all_strategies_map.insert(strategy.name.clone(), strategy.clone());
            }

            // Backtestear estrategias evolucionadas
            if config.verbose {
                println!("   🔄 Backtesteando estrategias evolucionadas...");
            }

            let evolved_results = match engine.run_massive_backtest(evolved_strategies.clone(), candles.clone(), &backtest_config).await {
                Ok(results) => {
                    if config.verbose {
                        println!("   ✅ {} estrategias evolucionadas backtesteadas", results.len());
                    }
                    results
                }
                Err(e) => {
                    eprintln!("   ⚠️  Error al backtestear estrategias evolucionadas: {}", e);
                    Vec::new()
                }
            };

            // Combinar resultados originales y evolucionados
            all_results.extend(evolved_results);

            if config.verbose {
                println!("   ✅ Total estrategias (originales + evolucionadas): {}\n", all_results.len());
            }
        }
    }

    // ==========================================
    // FASE 7: Re-filtrar y Re-ranquear (si hubo evolución)
    // ==========================================
    let final_top_strategies = if config.evolve.is_some() {
        if config.verbose {
            println!("🏆 FASE 7: Re-filtrando y re-ranqueando todas las estrategias...");
        }

        // Re-filtrar
        let re_filtered: Vec<&BacktestResult> = all_results
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

        // Re-ranquear
        let mut re_ranked: Vec<(f64, &BacktestResult)> = re_filtered
            .iter()
            .map(|r| {
                let m = &r.metrics;
                let weights = &config.score_weights;
                
                // Normalizar métricas
                let sharpe_norm = (m.sharpe_ratio + 2.0) / 4.0;
                let sortino_norm = (m.sortino_ratio + 2.0) / 4.0;
                let pf_norm = (m.profit_factor.min(5.0)) / 5.0;
                let return_norm = (m.total_return + 1.0) / 2.0;
                let dd_norm = 1.0 - m.max_drawdown_percent.min(1.0);
                
                let score = weights[0] * sharpe_norm +
                    weights[1] * sortino_norm +
                    weights[2] * pf_norm +
                    weights[3] * return_norm +
                    weights[4] * dd_norm;
                
                (score, *r)
            })
            .collect();

        re_ranked.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        
        let final_top: Vec<&BacktestResult> = re_ranked
            .iter()
            .take(config.top)
            .map(|(_, r)| *r)
            .collect();

        if config.verbose {
            println!("   ✅ Top {} estrategias finales seleccionadas\n", final_top.len());
        }

        final_top
    } else {
        top_strategies
    };

    // ==========================================
    // FASE 8: Mostrar Resultados
    // ==========================================
    if config.verbose {
        println!("📈 FASE 6: Top {} Estrategias", config.show_top);
        println!("{}", "=".repeat(100));
    }
    
    for (i, result) in final_top_strategies.iter().take(config.show_top).enumerate() {
        let m = &result.metrics;
        if config.verbose {
            println!("\n{}. {}", i + 1, result.strategy_name);
            println!("   📊 Métricas:");
            println!("      Total Return:     {:.2}%", m.total_return * 100.0);
            println!("      ROI sobre Capital Arriesgado: {:.2}%", m.return_on_risk * 100.0);
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
            if m.stop_loss_exits > 0 || m.take_profit_exits > 0 {
                println!("      Exits por Stop Loss:  {}", m.stop_loss_exits);
                println!("      Exits por Take Profit: {}", m.take_profit_exits);
            }
            println!("      Exits por Señal:    {}", m.signal_exits);
            if m.end_of_data_exits > 0 {
                println!("      Exits fin de datos:  {}", m.end_of_data_exits);
            }
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
    if config.evolve.is_some() {
        println!("   Evolucionadas:          {} generaciones", config.evolve.unwrap());
        println!("   Total backtesteadas:    {} (originales + evolucionadas)", all_results.len());
    } else {
        println!("   Backtesteadas:         {}", all_results.len());
    }
    let filtered_count_str = if config.evolve.is_some() { 
        "ver FASE 7".to_string() 
    } else { 
        format!("{}", filtered.len()) 
    };
    println!("   Pasaron filtros:       {}", filtered_count_str);
    println!("   Top {} seleccionadas: {}", config.top, final_top_strategies.len());

    // ==========================================
    // FASE 7: Guardar en SQLite (persistencia principal)
    // ==========================================
    if !config.no_db {
        if config.verbose {
            println!("\n💾 FASE 7: Guardando mejores estrategias en SQLite...");
        }

        // Inicializar base de datos
        let pool = match init_sqlite(&config.db_path).await {
            Ok(pool) => {
                if config.verbose {
                    println!("   ✅ Base de datos conectada: {}", config.db_path);
                }
                pool
            }
            Err(e) => {
                eprintln!("   ❌ Error al conectar a SQLite: {}", e);
                eprintln!("   💡 Continuando sin guardar en base de datos...");
                return if config.output.is_some() {
                    // Continuar para guardar JSON si está especificado
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("No se pudo guardar en SQLite y no se especificó --output"))
                };
            }
        };

        let strategy_repo = StrategyRepository::new(pool.clone());
        let backtest_repo = BacktestRepository::new(pool);

        // Preparar metadata de ejecución
        let execution_metadata = serde_json::json!({
            "data_file": config.data,
            "start_date": config.start_date,
            "end_date": config.end_date,
            "strategies_generated": config.strategies,
            "top_n": config.top,
            "filters": {
                "min_trades": config.min_trades,
                "min_win_rate": config.min_win_rate,
                "min_sharpe": config.min_sharpe,
                "min_return": config.min_return,
                "max_drawdown": config.max_drawdown,
            },
            "score_weights": weights,
            "backtest_config": {
                "initial_balance": config.initial_balance,
                "commission_rate": config.commission_rate,
                "slippage_bps": config.slippage_bps,
                "risk_per_trade": config.risk_per_trade,
                "stop_loss": config.stop_loss,
                "take_profit": config.take_profit,
            },
            "executed_at": chrono::Utc::now().to_rfc3339(),
        });

        // Guardar cada estrategia top
        let mut saved_strategy_ids = Vec::new();
        let mut saved_count = 0;
        let mut updated_count = 0;

        for (_i, result) in final_top_strategies.iter().enumerate() {
                if let Some(strategy_ast) = all_strategies_map.get(&result.strategy_name) {
                // Convertir AST a modelo
                let strategy_model = strategy_ast_to_model(
                    strategy_ast,
                    Some(&result.metrics),
                    Some(execution_metadata.clone()),
                );

                // Guardar o actualizar estrategia
                match strategy_repo.create_or_update_best(&strategy_model).await {
                    Ok(strategy_id) => {
                        saved_strategy_ids.push(strategy_id);
                        
                        // Verificar si fue nueva o actualizada
                        if let Some(existing) = strategy_repo.find_by_hash(
                            &strategy_model.strategy_hash.as_ref().unwrap()
                        ).await? {
                            if existing.id == Some(strategy_id) {
                                saved_count += 1;
                            } else {
                                updated_count += 1;
                            }
                        } else {
                            saved_count += 1;
                        }

                        // Guardar resultado de backtest extendido
                        let start_date_str = config.start_date.as_ref()
                            .map(|d| d.clone())
                            .unwrap_or_else(|| "N/A".to_string());
                        let end_date_str = config.end_date.as_ref()
                            .map(|d| d.clone())
                            .unwrap_or_else(|| "N/A".to_string());
                        
                        // Extraer timeframe del nombre del archivo o usar "unknown"
                        let timeframe = if config.data.contains("_1h") {
                            "1h"
                        } else if config.data.contains("_4h") {
                            "4h"
                        } else if config.data.contains("_1d") {
                            "1d"
                        } else {
                            "unknown"
                        };

                        // Calcular score para este resultado
                        let m = &result.metrics;
                        let weights = &config.score_weights;
                        let sharpe_norm = (m.sharpe_ratio + 2.0) / 4.0;
                        let sortino_norm = (m.sortino_ratio + 2.0) / 4.0;
                        let pf_norm = (m.profit_factor.min(5.0)) / 5.0;
                        let return_norm = (m.total_return + 1.0) / 2.0;
                        let dd_norm = 1.0 - m.max_drawdown_percent.min(1.0);
                        let composite_score = weights[0] * sharpe_norm +
                            weights[1] * sortino_norm +
                            weights[2] * pf_norm +
                            weights[3] * return_norm +
                            weights[4] * dd_norm;

                        let backtest_result = darwinx_store::BacktestResult {
                            id: None,
                            strategy_id,
                            dataset: config.data.clone(),
                            timeframe: timeframe.to_string(),
                            start_date: start_date_str,
                            end_date: end_date_str,
                            total_return: result.metrics.total_return,
                            sharpe_ratio: result.metrics.sharpe_ratio,
                            sortino_ratio: Some(result.metrics.sortino_ratio),
                            max_drawdown: result.metrics.max_drawdown,
                            win_rate: result.metrics.win_rate,
                            profit_factor: Some(result.metrics.profit_factor),
                            total_trades: result.trades.len() as i32,
                            tested_at: Some(chrono::Utc::now().to_rfc3339()),
                            annualized_return: Some(result.metrics.annualized_return),
                            max_drawdown_percent: Some(result.metrics.max_drawdown_percent),
                            total_profit: Some(result.metrics.total_profit),
                            total_loss: Some(result.metrics.total_loss),
                            max_consecutive_wins: Some(result.metrics.max_consecutive_wins as i32),
                            max_consecutive_losses: Some(result.metrics.max_consecutive_losses as i32),
                            trades_per_month: Some(result.metrics.trades_per_month),
                            trades_per_year: Some(result.metrics.trades_per_year),
                            stop_loss_exits: Some(result.metrics.stop_loss_exits as i32),
                            take_profit_exits: Some(result.metrics.take_profit_exits as i32),
                            signal_exits: Some(result.metrics.signal_exits as i32),
                            end_of_data_exits: Some(result.metrics.end_of_data_exits as i32),
                            composite_score: Some(composite_score),
                        };

                        match backtest_repo.create_or_update_extended(&backtest_result).await {
                            Ok(_) => {
                                // Silenciosamente actualizado o creado
                            }
                            Err(e) => {
                                eprintln!("   ⚠️  Error al guardar backtest result para {}: {}", result.strategy_name, e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("   ⚠️  Error al guardar estrategia {}: {}", result.strategy_name, e);
                    }
                }
            }
        }

        // Marcar todas las guardadas como mejores
        if !saved_strategy_ids.is_empty() {
            if let Err(e) = strategy_repo.mark_as_best(&saved_strategy_ids).await {
                eprintln!("   ⚠️  Error al marcar estrategias como mejores: {}", e);
            }
        }

        if config.verbose {
            println!("   ✅ Guardadas {} estrategias nuevas", saved_count);
            if updated_count > 0 {
                println!("   ✅ Actualizadas {} estrategias existentes", updated_count);
            }
            println!("   ✅ Total {} estrategias marcadas como mejores", saved_strategy_ids.len());
            println!("   📊 Base de datos: {}", config.db_path);
        } else {
            println!("\n💾 Guardadas {} estrategias en SQLite: {}", saved_strategy_ids.len(), config.db_path);
        }
    } else if config.verbose {
        println!("\n⏭️  Saltando guardado en SQLite (--no-db especificado)");
    }

    // ==========================================
    // FASE 8: Guardar Resultados JSON (opcional, solo para consulta rápida)
    // ==========================================
    if let Some(output_path) = &config.output {
        if config.verbose {
            println!("\n💾 Guardando resultados en {}...", output_path);
        }
        
        // Determinar timeframe del dataset para corregir el AST en el JSON
        use darwinx_core::TimeFrame;
        let dataset_timeframe = if config.data.contains("_1h") {
            TimeFrame::H1
        } else if config.data.contains("_4h") {
            TimeFrame::H4
        } else if config.data.contains("_1d") {
            TimeFrame::D1
        } else if config.data.contains("_15m") {
            TimeFrame::M15
        } else if config.data.contains("_30m") {
            TimeFrame::M30
        } else {
            TimeFrame::H1 // Default
        };
        
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
                "top_selected": final_top_strategies.len(),
            },
            "top_strategies": final_top_strategies.iter().enumerate().map(|(i, r)| {
                // Obtener la definición completa de la estrategia (AST)
                let mut strategy_ast = all_strategies_map.get(&r.strategy_name).cloned();
                
                // CORRECCIÓN: Sobrescribir el timeframe de la estrategia con el del dataset
                if let Some(ref mut ast) = strategy_ast {
                    ast.timeframe = dataset_timeframe;
                }
                
                // Calcular score
                let m = &r.metrics;
                let weights = &config.score_weights;
                let sharpe_norm = (m.sharpe_ratio + 2.0) / 4.0;
                let sortino_norm = (m.sortino_ratio + 2.0) / 4.0;
                let pf_norm = (m.profit_factor.min(5.0)) / 5.0;
                let return_norm = (m.total_return + 1.0) / 2.0;
                let dd_norm = 1.0 - m.max_drawdown_percent.min(1.0);
                let score = weights[0] * sharpe_norm +
                    weights[1] * sortino_norm +
                    weights[2] * pf_norm +
                    weights[3] * return_norm +
                    weights[4] * dd_norm;
                
                // Solo guardar métricas esenciales, NO trades ni equity_curve para reducir tamaño
                serde_json::json!({
                    "rank": i + 1,
                    "score": score,
                    "strategy_name": r.strategy_name,
                    "strategy": strategy_ast, // AST necesario para reproducir la estrategia
                    "metrics": r.metrics,     // Solo métricas, NO incluye trades ni equity_curve
                    "total_trades": r.trades.len(),
                    // NOTA: trades y equity_curve no se guardan en JSON para reducir tamaño
                    // Los trades completos están disponibles en SQLite si se necesitan
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
            println!("   ✅ Resultados JSON guardados en {} (solo para consulta rápida)", output_path);
            println!("   💡 Nota: SQLite es la persistencia principal. JSON es opcional.");
        } else {
            println!("💾 Resultados JSON guardados en {} (consulta rápida)", output_path);
        }
    }

    Ok(())
}

