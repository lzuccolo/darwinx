#!/bin/bash
# Script para ejecutar el binario massive_backtest
#
# CONFIGURACIÓN: Edita las variables abajo para ajustar los parámetros del backtest

set -e

# ============================================================================
# CONFIGURACIÓN DEL BACKTEST
# ============================================================================
# Edita estos valores según tus necesidades

# Archivo de datos (CSV o Parquet)
DATA_FILE="data/BTCUSDT_15m.parquet"

# Fechas del backtest (formato: YYYY-MM-DD, o deja vacío para usar todo el archivo)
START_DATE="2024-12-01"  # Ejemplo: "2024-01-01"
END_DATE="2025-03-01"    # Ejemplo: "2024-12-31"

# Número de estrategias a generar
STRATEGIES=30000

# Top N estrategias a seleccionar
TOP=100

# Balance inicial
INITIAL_BALANCE=1000.0

# Comisión por trade (como porcentaje, ej: 0.001 = 0.1%)
COMMISSION_RATE=0.001

# Slippage en basis points (ej: 5 = 0.05%)
SLIPPAGE_BPS=5.0

# Porcentaje del balance a usar por posición (ej: 0.5 = 50%, 0.95 = 95%)
# Este es el tamaño FIJO de cada posición para comparar estrategias de forma justa
# Tamaño = (balance * POSITION_SIZE) / max_positions
POSITION_SIZE=0.95

# Stop Loss como porcentaje del precio de entrada (ej: 0.05 = 5%, deja vacío para deshabilitar)
STOP_LOSS=""

# Take Profit como porcentaje del precio de entrada (ej: 0.10 = 10%, deja vacío para deshabilitar)
TAKE_PROFIT=""

# Filtros de calidad
# NOTA: Estos filtros son más realistas para permitir que pasen estrategias viables
# Puedes ajustarlos según tus necesidades
MIN_TRADES=10          # Mínimo de trades para considerar la estrategia válida
MIN_WIN_RATE=0.40      # Win rate mínimo (40% - más permisivo)
MIN_SHARPE=0.0         # Sharpe mínimo (0.0 = sin filtro de Sharpe)
MIN_RETURN=0.10        # Retorno mínimo del 10% sobre balance inicial
MAX_DRAWDOWN=0.4       # Drawdown máximo del 40% (más permisivo)

# Evolución Genética (deja vacío para deshabilitar)
EVOLVE_GENERATIONS=""  # Ejemplo: "50" para 50 generaciones
EVOLVE_POPULATION=100  # Tamaño de población para evolución
EVOLVE_MUTATION_RATE=0.1  # Tasa de mutación (0.0-1.0)
EVOLVE_ELITE_SIZE=10  # Tamaño de elite preservado

# Cargar mejores estrategias históricas desde SQLite (deja vacío para deshabilitar)
LOAD_BEST=""  # Ejemplo: "50" para cargar 50 mejores estrategias

# Pesos para el score compuesto (Sharpe, Sortino, Profit Factor, Return, Drawdown)
# Formato: 5 valores separados por comas
SCORE_WEIGHTS="0.3,0.2,0.2,0.15,0.15"

# Mostrar top N estrategias en consola
SHOW_TOP=10

# Modo verbose (true/false)
VERBOSE=true

# Variables derivadas del archivo de datos
DATA_BASENAME="$(basename "$DATA_FILE")"
PAIR="${DATA_BASENAME%%_*}"
TIMEFRAME="${DATA_BASENAME#*_}"
TIMEFRAME="${TIMEFRAME%%.*}"
EXECUTION_DATE="$(date +%Y%m%d_%H%M%S)"

# Guardar resultados en archivo JSON (incluye par, timeframe, fechas y fecha de ejecución)
OUTPUT_FILE="results/massive_backtest_${PAIR}_${TIMEFRAME}_${START_DATE:-all}_${END_DATE:-all}_${EXECUTION_DATE}.json"

# ============================================================================
# CÓDIGO DEL SCRIPT (no editar a menos que sepas lo que haces)
# ============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY="$SCRIPT_DIR/massive_backtest"

# Verificar que el binario existe
if [ ! -f "$BINARY" ]; then
    echo "❌ Error: El binario no existe en $BINARY"
    echo "💡 Ejecuta primero: ./scripts/build-binaries.sh"
    exit 1
fi

# Construir argumentos del comando
ARGS=()

# Evolución genética
if [ -n "$EVOLVE_GENERATIONS" ]; then
    ARGS+=(--evolve "$EVOLVE_GENERATIONS")
    ARGS+=(--evolve-population "$EVOLVE_POPULATION")
    ARGS+=(--evolve-mutation-rate "$EVOLVE_MUTATION_RATE")
    ARGS+=(--evolve-elite-size "$EVOLVE_ELITE_SIZE")
fi

# Cargar mejores estrategias históricas
if [ -n "$LOAD_BEST" ]; then
    ARGS+=(--load-best "$LOAD_BEST")
fi

ARGS+=("--strategies" "$STRATEGIES")
ARGS+=("--data" "$DATA_FILE")
ARGS+=("--top" "$TOP")
ARGS+=("--initial-balance" "$INITIAL_BALANCE")
ARGS+=("--commission-rate" "$COMMISSION_RATE")
ARGS+=("--slippage-bps" "$SLIPPAGE_BPS")
ARGS+=("--position-size" "$POSITION_SIZE")

# Agregar stop loss si está configurado
if [ -n "$STOP_LOSS" ]; then
    ARGS+=("--stop-loss" "$STOP_LOSS")
fi

# Agregar take profit si está configurado
if [ -n "$TAKE_PROFIT" ]; then
    ARGS+=("--take-profit" "$TAKE_PROFIT")
fi

ARGS+=("--min-trades" "$MIN_TRADES")
ARGS+=("--min-win-rate" "$MIN_WIN_RATE")
ARGS+=("--min-sharpe" "$MIN_SHARPE")
ARGS+=("--min-return" "$MIN_RETURN")
ARGS+=("--max-drawdown" "$MAX_DRAWDOWN")
ARGS+=("--show-top" "$SHOW_TOP")

# Score weights: pasar cada valor como argumento separado
IFS=',' read -ra WEIGHT_ARRAY <<< "$SCORE_WEIGHTS"
ARGS+=("--score-weights")
for weight in "${WEIGHT_ARRAY[@]}"; do
    ARGS+=("$weight")
done

# Agregar fechas solo si están configuradas
if [ -n "$START_DATE" ]; then
    ARGS+=("--start-date" "$START_DATE")
fi

if [ -n "$END_DATE" ]; then
    ARGS+=("--end-date" "$END_DATE")
fi

# Agregar output solo si está configurado
if [ -n "$OUTPUT_FILE" ]; then
    ARGS+=("--output" "$OUTPUT_FILE")
fi

# Agregar verbose si está habilitado
if [ "$VERBOSE" = "true" ]; then
    ARGS+=("--verbose")
fi

# Ejecutar el binario
exec "$BINARY" "${ARGS[@]}"
