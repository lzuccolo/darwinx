darwinx/
├── Cargo.toml                           # Workspace root
├── README.md
├── LICENSE
├── .gitignore
├── rustfmt.toml
├── clippy.toml
│
├── crates/
│   ├── core/
│   ├── indicators/
│   ├── data/
│   ├── strategy-store/
│   ├── strategy-generator/
│   ├── backtest-engine/
│   ├── strategy-converter/
│   ├── optimizer/
│   ├── runner-live/
│   ├── data-manager/
│   ├── api-proto/
│   ├── api-server/
│   ├── api-client/
│   ├── cli-client/
│   └── gui-client/
│
├── config/
├── data/
├── strategies/
├── logs/
└── docs/

crates/strategy-generator/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── ast.rs              ← AST nodes y builder
    ├── ast/
    │   ├── nodes.rs
    │   ├── builder.rs
    │   └── validator.rs
    ├── generator.rs        ← Generadores
    ├── generator/
    │   ├── random.rs
    │   └── genetic.rs
    └── constraints.rs      ← Validaciones