#!/bin/bash
# Script para ejecutar el binario massive_backtest
# El binario se compila y copia a bin/massive_backtest

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BIN_DIR="$SCRIPT_DIR"
BINARY_IN_BIN="$BIN_DIR/massive_backtest"

# Detectar si estamos en modo release o debug
if [ "$1" = "--release" ] || [ "$1" = "-r" ]; then
    SOURCE_BINARY="$PROJECT_ROOT/target/release/massive_backtest"
    BUILD_MODE="release"
    shift  # Remover --release de los argumentos
else
    SOURCE_BINARY="$PROJECT_ROOT/target/debug/massive_backtest"
    BUILD_MODE="debug"
fi

# Función para compilar
build() {
    echo "🔨 Compilando massive_backtest en modo $BUILD_MODE..."
    cd "$PROJECT_ROOT"
    if [ "$BUILD_MODE" = "release" ]; then
        cargo build --release --bin massive_backtest
    else
        cargo build --bin massive_backtest
    fi
    
    if [ $? -ne 0 ]; then
        echo "❌ Error al compilar"
        exit 1
    fi
    
    # Copiar binario a bin/ solo en modo release
    if [ "$BUILD_MODE" = "release" ]; then
        echo "📦 Copiando binario a $BIN_DIR..."
        cp "$SOURCE_BINARY" "$BINARY_IN_BIN"
        chmod +x "$BINARY_IN_BIN"
        echo "✅ Binario copiado a $BINARY_IN_BIN"
    fi
}

# Si el binario fuente no existe o necesita recompilarse
if [ ! -f "$SOURCE_BINARY" ]; then
    build
elif [ "$BUILD_MODE" = "release" ] && ([ ! -f "$BINARY_IN_BIN" ] || [ "$SOURCE_BINARY" -nt "$BINARY_IN_BIN" ]); then
    build
fi

# Ejecutar el binario
# En modo release: desde bin/, en modo debug: desde target/
if [ "$BUILD_MODE" = "release" ] && [ -f "$BINARY_IN_BIN" ]; then
    echo "🚀 Ejecutando massive_backtest (release desde bin/)..."
    exec "$BINARY_IN_BIN" "$@"
else
    echo "🚀 Ejecutando massive_backtest (debug desde target/)..."
    exec "$SOURCE_BINARY" "$@"
fi

