#!/usr/bin/env bash
# Ejecuta el binario export_strategy para varios strategy_name.
# Edita la lista STRATEGY_NAMES antes de usar.

set -euo pipefail

# ============================================================================
# CONFIGURA AQUÍ LOS NOMBRES DE ESTRATEGIA A EXPORTAR
# Ejemplo: los strategy_name que ves en los JSON de resultados.
# ============================================================================
STRATEGY_NAMES=(
  "Strategy_39839"
  # Añade más nombres abajo...
  # "Strategy_XXXXX"
)

# Directorio de salida (puede sobreescribirse con la variable de entorno OUT_DIR)
OUT_DIR="${OUT_DIR:-exports}"

# Ruta a la base de datos SQLite (puede sobreescribirse con DB_PATH)
DB_PATH="${DB_PATH:-data/strategies.db}"

# ============================================================================
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

mkdir -p "$REPO_ROOT/$OUT_DIR"

for name in "${STRATEGY_NAMES[@]}"; do
  echo "Exportando estrategia: ${name}"
  cargo run --bin export_strategy \
    -- --strategy-name "$name" \
    --db-path "$REPO_ROOT/$DB_PATH" \
    --out-dir "$REPO_ROOT/$OUT_DIR"
done

echo "✅ Exportación finalizada. Archivos en: $REPO_ROOT/$OUT_DIR"

