# Strategy Converter Hub

Hub central de conversión de estrategias entre diferentes formatos.

## 🎯 Objetivo

Proporcionar conversión bidireccional entre:
- **AST** (Abstract Syntax Tree) - Formato interno
- **Rhai** - DSL para estrategias manuales
- **Rust** - Código que implementa el trait `Strategy`
- **Python** - Scripts Python
- **Freqtrade** - Formato de estrategias Freqtrade

## 📦 Estructura

```
strategy-converter/
├── src/
│   ├── lib.rs              # API pública
│   ├── error.rs            # Manejo de errores
│   ├── formats.rs          # Definición de formatos
│   ├── converter.rs        # Trait y implementación principal
│   ├── inputs/             # Parsers (a AST)
│   │   └── rhai_parser.rs
│   └── outputs/            # Generadores (desde AST)
│       ├── rhai_generator.rs
│       ├── rust_generator.rs
│       ├── python_generator.rs
│       └── freqtrade_generator.rs
```

## 🚀 Uso Básico

```rust
use darwinx_converter::{DefaultStrategyConverter, StrategyFormat};

let converter = DefaultStrategyConverter::new();

// Convertir Rhai a AST
let ast = converter.from_format(rhai_script, StrategyFormat::Rhai)?;

// Convertir AST a Rust
let rust_code = converter.to_format(&ast, StrategyFormat::Rust)?;

// Conversión directa entre formatos
let python_code = converter.convert(
    rhai_script,
    StrategyFormat::Rhai,
    StrategyFormat::Python
)?;
```

## 📋 Estado de Implementación

| Formato | Parser (→ AST) | Generador (AST →) | Estado |
|---------|---------------|-------------------|--------|
| AST     | ✅ JSON       | ✅ JSON           | ✅ Completo |
| Rhai    | ⏳ Pendiente | ⏳ Pendiente      | 🚧 En desarrollo |
| Rust    | ❌ No soportado | ⏳ Pendiente      | 🚧 En desarrollo |
| Python  | ❌ No soportado | ⏳ Pendiente      | 🚧 En desarrollo |
| Freqtrade | ❌ No soportado | ⏳ Pendiente      | 🚧 En desarrollo |

## 🔧 Próximos Pasos

1. **Parser Rhai** - Implementar parsing completo de scripts Rhai
2. **Generador Rhai** - Generar scripts Rhai desde AST
3. **Generador Rust** - Generar código Rust compilable
4. **Generador Python** - Generar scripts Python ejecutables
5. **Generador Freqtrade** - Generar estrategias Freqtrade

## 📝 Notas

- El converter usa AST como formato intermedio para todas las conversiones
- Las conversiones bidireccionales solo están disponibles cuando ambos formatos lo soportan
- Todos los parsers y generadores validan la estructura antes de convertir

