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
DATA_FILE="data/BTCUSDT_1h.parquet"

# Fechas del backtest (formato: YYYY-MM-DD, o deja vacío para usar todo el archivo)
START_DATE="2024-12-01"  # Ejemplo: "2024-01-01"
END_DATE="2025-03-01"    # Ejemplo: "2024-12-31"

# Número de estrategias a generar
STRATEGIES=100000

# Top N estrategias a seleccionar
TOP=100

# Balance inicial
INITIAL_BALANCE=1000.0

# Comisión por trade (como porcentaje, ej: 0.001 = 0.1%)
COMMISSION_RATE=0.001

# Slippage en basis points (ej: 5 = 0.05%)
SLIPPAGE_BPS=5.0

# Riesgo por trade como porcentaje del balance (ej: 0.02 = 2%)
RISK_PER_TRADE=0.02

# Stop Loss como porcentaje del precio de entrada (ej: 0.02 = 2%, deja vacío para deshabilitar)
STOP_LOSS=""  # Ejemplo: "0.02"

# Take Profit como porcentaje del precio de entrada (ej: 0.05 = 5%, deja vacío para deshabilitar)
TAKE_PROFIT=""  # Ejemplo: "0.05"

# Filtros de calidad
MIN_TRADES=10
MIN_WIN_RATE=0.5
MIN_SHARPE=0.0
MIN_RETURN=0.00
MAX_DRAWDOWN=0.5

# Pesos para el score compuesto (Sharpe, Sortino, Profit Factor, Return, Drawdown)
# Formato: 5 valores separados por comas
SCORE_WEIGHTS="0.3,0.2,0.2,0.15,0.15"

# Mostrar top N estrategias en consola
SHOW_TOP=10

# Guardar resultados en archivo JSON (deja vacío para no guardar)
OUTPUT_FILE="results/massive_backtest_btcusdt_1h_2024-12-01_2025-03-01_100000_100_100000_0.001_5_0.02_10_0.5_0.0_0.01_0.5_0.3,0.2,0.2,0.15,0.15_10.json"  # Ejemplo: "resultados_backtest.json"

# Modo verbose (true/false)
VERBOSE=true

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

ARGS+=("--strategies" "$STRATEGIES")
ARGS+=("--data" "$DATA_FILE")
ARGS+=("--top" "$TOP")
ARGS+=("--initial-balance" "$INITIAL_BALANCE")
ARGS+=("--commission-rate" "$COMMISSION_RATE")
ARGS+=("--slippage-bps" "$SLIPPAGE_BPS")
ARGS+=("--risk-per-trade" "$RISK_PER_TRADE")

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
