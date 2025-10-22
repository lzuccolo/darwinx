#!/bin/bash
# Script: extract_darwinx_code.sh

OUTPUT_FILE="/home/shared/trading/src/darwinx/tmp/darwinx_code_snapshot.txt"

echo "🔍 Extrayendo código de DarwinX..." > "$OUTPUT_FILE"
echo "Fecha: $(date)" >> "$OUTPUT_FILE"
echo "======================================" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Función para agregar un archivo
add_file() {
    local file=$1
    if [ -f "$file" ]; then
        echo "" >> "$OUTPUT_FILE"
        echo "========================================" >> "$OUTPUT_FILE"
        echo "📄 FILE: $file" >> "$OUTPUT_FILE"
        echo "========================================" >> "$OUTPUT_FILE"
        cat "$file" >> "$OUTPUT_FILE"
        echo "" >> "$OUTPUT_FILE"
    fi
}

# Cambiar al directorio del proyecto
cd /home/shared/trading/src/darwinx

# Indicators (archivos clave)
add_file "crates/indicators/src/lib.rs"
add_file "crates/indicators/src/metadata.rs"
add_file "crates/indicators/Cargo.toml"

# Un indicador de cada categoría (ejemplo)
add_file "crates/indicators/src/trend/sma.rs"
add_file "crates/indicators/src/momentum/rsi.rs"
add_file "crates/indicators/src/volatility/bollinger.rs"
add_file "crates/indicators/src/volume/obv.rs"

# Archivos de módulos
add_file "crates/indicators/src/trend.rs"
add_file "crates/indicators/src/momentum.rs"

# Strategy generator
add_file "crates/strategy-generator/src/lib.rs"
add_file "crates/strategy-generator/src/generator/random.rs"
add_file "crates/strategy-generator/Cargo.toml"

# Core (para contexto)
add_file "crates/core/src/types/candle.rs"

echo "" >> "$OUTPUT_FILE"
echo "======================================" >> "$OUTPUT_FILE"
echo "✅ Extracción completada" >> "$OUTPUT_FILE"
echo "Archivo generado: $OUTPUT_FILE" >> "$OUTPUT_FILE"

# Mostrar tamaño del archivo
FILE_SIZE=$(wc -c < "$OUTPUT_FILE" | numfmt --to=iec)
echo "📊 Tamaño: $FILE_SIZE" >> "$OUTPUT_FILE"

echo "✅ Archivo generado: $OUTPUT_FILE"
echo "📊 Tamaño: $FILE_SIZE"
echo ""
echo "Puedes adjuntarlo o hacer: cat $OUTPUT_FILE"