# Binarios DarwinX

Este directorio contiene los binarios compilados y scripts de ejecución.

## Estructura

```
bin/
├── massive_backtest          # Binario compilado (generado automáticamente)
├── massive_backtest.sh       # Script de ejecución (compila y ejecuta si es necesario)
└── README.md                  # Este archivo
```

## Uso

### Opción 1: Script de ejecución (recomendado para desarrollo)

El script `massive_backtest.sh` compila automáticamente si es necesario y ejecuta el binario:

```bash
# Modo debug (por defecto) - CSV
./bin/massive_backtest.sh --strategies 10000 --data data/btcusdt_1h.csv

# Modo release - Parquet (recomendado, más rápido)
./bin/massive_backtest.sh --release --strategies 10000 --data data/BTCUSDT_1h.parquet

# También puedes usar -d como atajo
./bin/massive_backtest.sh --release -d data/BTCUSDT_1h.parquet --strategies 10000
```

### Opción 2: Script de build (recomendado para producción/testing)

Compila y copia los binarios a `bin/` (solo en modo release):

```bash
# Modo release (por defecto, copia a bin/)
./scripts/build-binaries.sh

# Modo debug (no copia a bin/, solo compila)
./scripts/build-binaries.sh --debug
```

Luego ejecuta directamente:

```bash
# CSV
./bin/massive_backtest --strategies 10000 --data data/btcusdt_1h.csv

# Parquet (recomendado, más eficiente)
./bin/massive_backtest --strategies 10000 --data data/BTCUSDT_1h.parquet
```

### Formatos de Datos Soportados

El CLI detecta automáticamente el formato del archivo por su extensión:

- **CSV** (`.csv`): Formato texto, fácil de inspeccionar
- **Parquet** (`.parquet`): Formato binario, más eficiente y rápido de cargar (recomendado)

Ejemplo de formato esperado (mismo para ambos):
```
timestamp,open,high,low,close,volume
1609459200000,29000.0,29500.0,28800.0,29200.0,1500.5
```

**Nota importante**: Usa `--data` o `-d` para especificar el archivo. El formato se detecta automáticamente.

### Evolución Genética

El CLI soporta evolución genética de estrategias después del backtest inicial:

```bash
# Evolución genética con 50 generaciones
./bin/massive_backtest.sh --evolve 50

# Con parámetros personalizados
./bin/massive_backtest.sh \
  --evolve 100 \
  --evolve-population 200 \
  --evolve-mutation-rate 0.15 \
  --evolve-elite-size 20
```

**Opciones de evolución:**
- `--evolve N`: Habilita evolución genética con N generaciones
- `--evolve-population SIZE`: Tamaño de población (default: 100)
- `--evolve-mutation-rate RATE`: Tasa de mutación 0.0-1.0 (default: 0.1)
- `--evolve-elite-size SIZE`: Tamaño de elite preservado (default: 10)

**Flujo de evolución:**
1. Backtest inicial de estrategias generadas
2. Selección de top estrategias
3. Evolución genética de las mejores (crossover + mutación)
4. Backtest de estrategias evolucionadas
5. Re-filtrado y re-ranqueo de todas (originales + evolucionadas)
6. Guardado de mejores finales en SQLite

### Cargar Mejores Estrategias Históricas

Puedes cargar las mejores estrategias guardadas en SQLite para usarlas como población inicial:

```bash
# Cargar 50 mejores estrategias históricas y generar 50 adicionales aleatorias
./bin/massive_backtest.sh --load-best 50 --strategies 100

# Combinar con evolución genética
./bin/massive_backtest.sh --load-best 50 --strategies 100 --evolve 30
```

**Nota**: `--load-best N` carga N estrategias desde SQLite. Si `--strategies` es mayor, genera el resto aleatoriamente.

### Opción 3: Cargo directamente

```bash
# Desarrollo
cargo run --bin massive_backtest -- --help

# Release
cargo build --release --bin massive_backtest
./target/release/massive_backtest --help
```

## Para Producción/Testing

### En servidores de testing/producción:

1. **Compilar en CI/CD o máquina de build:**
   ```bash
   ./scripts/build-binaries.sh --release
   ```

2. **Copiar el directorio `bin/` completo:**
   ```bash
   scp -r bin/ user@prod-server:/opt/darwinx/bin/
   ```

3. **En el servidor, ejecutar directamente:**
   ```bash
   /opt/darwinx/bin/massive_backtest --strategies 10000 --data /data/btcusdt_1h.csv
   ```

### Ventajas de este enfoque:

- ✅ No requiere Rust instalado en producción
- ✅ Binarios optimizados (release mode)
- ✅ Fácil de empaquetar y distribuir
- ✅ Scripts de ejecución incluidos
- ✅ No depende de `target/` directory

## Opciones Avanzadas

### Filtrado por Fechas

```bash
# Backtest solo en un rango de fechas
./bin/massive_backtest.sh \
  --start-date 2024-01-01 \
  --end-date 2024-12-31 \
  --data data/BTCUSDT_1h.parquet
```

### Stop Loss y Take Profit

```bash
# Stop Loss 2%, Take Profit 5%
./bin/massive_backtest.sh \
  --stop-loss 0.02 \
  --take-profit 0.05 \
  --data data/BTCUSDT_1h.parquet
```

### Persistencia en SQLite

Por defecto, las mejores estrategias se guardan en `data/strategies.db`:

```bash
# Cambiar ruta de base de datos
./bin/massive_backtest.sh --db-path /path/to/strategies.db

# No guardar en SQLite (solo JSON si se especifica --output)
./bin/massive_backtest.sh --no-db --output results.json
```

## Notas

- Los binarios en `bin/` se generan automáticamente al compilar
- El script `massive_backtest.sh` detecta si necesita recompilar comparando timestamps
- Para producción, siempre usar modo `--release` para mejor rendimiento
- SQLite es la fuente de verdad principal para persistencia de estrategias
- JSON (`--output`) es solo para consulta rápida de la ejecución actual

