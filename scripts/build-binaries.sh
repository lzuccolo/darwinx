#!/bin/bash
# Script para compilar y copiar todos los binarios a bin/
# Solo copia en modo release (por defecto)
# Uso: ./scripts/build-binaries.sh [--release]

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN_DIR="$PROJECT_ROOT/bin"

cd "$PROJECT_ROOT"

# Por defecto siempre en modo release para producción
if [ "$1" = "--debug" ] || [ "$1" = "-d" ]; then
    BUILD_MODE="debug"
    CARGO_FLAGS=""
    SOURCE_DIR="target/debug"
    COPY_TO_BIN=false
else
    BUILD_MODE="release"
    CARGO_FLAGS="--release"
    SOURCE_DIR="target/release"
    COPY_TO_BIN=true
fi

echo "🔨 Compilando binarios en modo $BUILD_MODE..."

# Compilar todos los binarios del crate cli
cargo build $CARGO_FLAGS --bin massive_backtest

if [ $? -ne 0 ]; then
    echo "❌ Error al compilar"
    exit 1
fi

# Copiar binarios a bin/ solo en modo release
if [ "$COPY_TO_BIN" = true ]; then
    # Asegurar que bin/ existe
    mkdir -p "$BIN_DIR"
    
    echo "📦 Copiando binarios a $BIN_DIR..."
    
    cp "$PROJECT_ROOT/$SOURCE_DIR/massive_backtest" "$BIN_DIR/massive_backtest"
    chmod +x "$BIN_DIR/massive_backtest"
    
    echo "✅ Binarios copiados a $BIN_DIR"
    echo "   - massive_backtest (modo $BUILD_MODE)"
else
    echo "ℹ️  Modo debug: binarios no copiados a bin/ (solo release se copia)"
    echo "   Binario disponible en: $PROJECT_ROOT/$SOURCE_DIR/massive_backtest"
fi

