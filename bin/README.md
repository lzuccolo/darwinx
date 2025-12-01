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
# Modo debug (por defecto)
./bin/massive_backtest.sh --strategies 10000 --data data/btcusdt_1h.csv

# Modo release
./bin/massive_backtest.sh --release --strategies 10000 --data data/btcusdt_1h.csv
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
./bin/massive_backtest --strategies 10000 --data data/btcusdt_1h.csv
```

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

## Notas

- Los binarios en `bin/` se generan automáticamente al compilar
- El script `massive_backtest.sh` detecta si necesita recompilar comparando timestamps
- Para producción, siempre usar modo `--release` para mejor rendimiento

