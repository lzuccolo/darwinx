darwinx/
├── Cargo.toml                           # Workspace root
├── README.md
├── ARCHITECTURE.md
│
├── crates/
│   │
│   ├── core/                            # 📦 Tipos y traits fundamentales
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── types.rs
│   │       ├── types/
│   │       │   ├── candle.rs
│   │       │   ├── signal.rs
│   │       │   ├── position.rs
│   │       │   ├── order.rs
│   │       │   └── timeframe.rs
│   │       ├── traits.rs
│   │       └── traits/
│   │           ├── strategy.rs
│   │           ├── market_data.rs
│   │           └── risk_manager.rs
│   │
│   ├── indicators/                      # 📊 Indicadores técnicos
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── trend.rs
│   │       ├── trend/
│   │       │   ├── sma.rs
│   │       │   ├── ema.rs
│   │       │   ├── wma.rs
│   │       │   └── vwma.rs
│   │       ├── momentum.rs
│   │       ├── momentum/
│   │       │   ├── rsi.rs
│   │       │   ├── macd.rs
│   │       │   ├── stochastic.rs
│   │       │   └── roc.rs
│   │       ├── volatility.rs
│   │       ├── volatility/
│   │       │   ├── bollinger.rs
│   │       │   ├── atr.rs
│   │       │   └── keltner.rs
│   │       ├── volume.rs
│   │       └── volume/
│   │           ├── obv.rs
│   │           ├── mfi.rs
│   │           └── vwap.rs
│   │
│   ├── data/                            # 💾 Manejo de datos
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── loader.rs
│   │       ├── loader/
│   │       │   ├── csv.rs
│   │       │   ├── parquet.rs
│   │       │   └── database.rs
│   │       ├── multi_timeframe.rs
│   │       └── multi_timeframe/
│   │           ├── context.rs
│   │           ├── synchronizer.rs
│   │           └── cache.rs
│   │
│   ├── strategy-store/                  # 🗄️ Base de datos
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── models.rs
│   │   │   ├── models/
│   │   │   │   ├── strategy.rs
│   │   │   │   ├── backtest_result.rs
│   │   │   │   └── trade.rs
│   │   │   ├── repositories.rs
│   │   │   └── repositories/
│   │   │       ├── strategy_repo.rs
│   │   │       ├── backtest_repo.rs
│   │   │       └── trade_repo.rs
│   │   └── migrations/
│   │
│   ├── strategy-generator/              # 🧬 Generador
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── generator.rs
│   │       ├── generator/
│   │       │   ├── random.rs
│   │       │   ├── genetic.rs
│   │       │   ├── grid.rs
│   │       │   └── grammar.rs
│   │       ├── ast.rs
│   │       ├── ast/
│   │       │   ├── nodes.rs
│   │       │   ├── builder.rs
│   │       │   └── validator.rs
│   │       ├── constraints.rs
│   │       └── constraints/
│   │           ├── complexity.rs
│   │           └── rules.rs
│   │
│   ├── backtest-engine/                 # ⚡ Motor de backtest
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── polars_engine.rs
│   │       ├── polars_engine/
│   │       │   ├── vectorized.rs
│   │       │   ├── parallel.rs
│   │       │   └── optimizer.rs
│   │       ├── event_driven.rs
│   │       ├── event_driven/
│   │       │   ├── engine.rs
│   │       │   ├── order_book.rs
│   │       │   └── execution.rs
│   │       ├── metrics.rs
│   │       ├── metrics/
│   │       │   ├── returns.rs
│   │       │   ├── risk.rs
│   │       │   └── statistics.rs
│   │       ├── batch.rs
│   │       └── batch/
│   │           ├── scheduler.rs
│   │           └── worker.rs
│   │
│   ├── strategy-converter/              # 🔄 Conversor
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── formats.rs
│   │       ├── formats/
│   │       │   ├── intermediate.rs
│   │       │   ├── rhai.rs
│   │       │   ├── rust.rs
│   │       │   ├── python.rs
│   │       │   └── freqtrade.rs
│   │       ├── compiler.rs
│   │       ├── compiler/
│   │       │   ├── codegen.rs
│   │       │   └── optimizer.rs
│   │       ├── parser.rs
│   │       └── parser/
│   │           ├── json_ast.rs
│   │           └── validator.rs
│   │
│   ├── runner-live/                     # 🔴 Ejecutor en vivo
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── engine.rs
│   │       ├── engine/
│   │       │   ├── rhai_runner.rs
│   │       │   ├── rust_runner.rs
│   │       │   └── executor.rs
│   │       ├── exchange.rs
│   │       ├── exchange/
│   │       │   ├── binance.rs
│   │       │   ├── bybit.rs
│   │       │   ├── okx.rs
│   │       │   └── mock.rs
│   │       ├── risk.rs
│   │       ├── risk/
│   │       │   ├── manager.rs
│   │       │   ├── kelly.rs
│   │       │   └── var.rs
│   │       ├── modes.rs
│   │       └── modes/
│   │           ├── demo.rs
│   │           ├── paper.rs
│   │           └── live.rs
│   │
│   ├── optimizer/                       # 🔧 Optimizador
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── algorithms.rs
│   │       ├── algorithms/
│   │       │   ├── grid_search.rs
│   │       │   ├── random_search.rs
│   │       │   ├── genetic.rs
│   │       │   ├── bayesian.rs
│   │       │   └── walk_forward.rs
│   │       ├── objective.rs
│   │       ├── objective/
│   │       │   ├── sharpe.rs
│   │       │   ├── sortino.rs
│   │       │   └── custom.rs
│   │       ├── space.rs
│   │       └── space/
│   │           ├── parameter.rs
│   │           └── bounds.rs
│   │
│   ├── data-manager/                    # 📊 Gestor de datos
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── downloaders.rs
│   │       ├── downloaders/
│   │       │   ├── binance.rs
│   │       │   ├── bybit.rs
│   │       │   └── yahoo_finance.rs
│   │       ├── converters.rs
│   │       ├── converters/
│   │       │   ├── csv_to_parquet.rs
│   │       │   └── parquet_to_csv.rs
│   │       ├── sync.rs
│   │       └── sync/
│   │           ├── daemon.rs
│   │           └── scheduler.rs
│   │
│   ├── api-proto/                       # 🌐 Protocol Buffers (gRPC)
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   └── proto/
│   │       ├── common.proto
│   │       ├── strategy.proto
│   │       ├── backtest.proto
│   │       ├── optimizer.proto
│   │       ├── live.proto
│   │       └── data.proto
│   │
│   ├── api-server/                      # 🖥️ Servidor gRPC
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── lib.rs
│   │       ├── server.rs
│   │       ├── services.rs
│   │       ├── services/
│   │       │   ├── strategy_service.rs
│   │       │   ├── backtest_service.rs
│   │       │   ├── optimizer_service.rs
│   │       │   ├── live_service.rs
│   │       │   └── data_service.rs
│   │       ├── auth.rs
│   │       └── middleware.rs
│   │
│   ├── api-client/                      # 🔌 Cliente gRPC
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── client.rs
│   │       ├── services.rs
│   │       └── services/
│   │           ├── strategy_client.rs
│   │           ├── backtest_client.rs
│   │           ├── optimizer_client.rs
│   │           ├── live_client.rs
│   │           └── data_client.rs
│   │
│   ├── cli-client/                      # 💻 Cliente CLI
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── commands.rs
│   │       ├── commands/
│   │       │   ├── generate.rs
│   │       │   ├── backtest.rs
│   │       │   ├── convert.rs
│   │       │   ├── run.rs
│   │       │   ├── analyze.rs
│   │       │   ├── optimize.rs
│   │       │   ├── list.rs
│   │       │   └── export.rs
│   │       ├── output.rs
│   │       ├── output/
│   │       │   ├── table.rs
│   │       │   ├── chart.rs
│   │       │   └── progress.rs
│   │       └── config.rs
│   │
│   └── gui-client/                      # 🖥️ GUI (GTK4/Relm4)
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── app.rs
│           ├── models.rs
│           ├── models/
│           │   ├── strategy.rs
│           │   ├── backtest.rs
│           │   └── state.rs
│           ├── messages.rs
│           ├── messages/
│           │   ├── app_msg.rs
│           │   └── view_msg.rs
│           ├── components.rs
│           ├── components/
│           │   ├── generator_view.rs
│           │   ├── backtest_view.rs
│           │   ├── live_view.rs
│           │   ├── analysis_view.rs
│           │   ├── strategy_card.rs
│           │   ├── chart_widget.rs
│           │   └── sidebar.rs
│           ├── services.rs
│           ├── services/
│           │   ├── strategy_service.rs
│           │   ├── backtest_service.rs
│           │   └── connection.rs
│           ├── widgets.rs
│           ├── widgets/
│           │   ├── custom_chart.rs
│           │   └── metric_card.rs
│           ├── utils.rs
│           └── utils/
│               └── formatters.rs
│
├── config/
│   ├── server.toml                      # Config del servidor
│   ├── client.toml                      # Config del cliente
│   └── database.toml
│
├── data/
│   ├── raw/
│   └── processed/
│
├── strategies/
│   ├── strategies.db
│   ├── intermediate/
│   ├── rhai/
│   └── compiled/
│
├── deploy/
│   ├── docker/
│   │   ├── Dockerfile.server
│   │   ├── Dockerfile.cli
│   │   └── Dockerfile.gui
│   ├── kubernetes/
│   │   ├── server-deployment.yaml
│   │   └── server-service.yaml
│   └── systemd/
│       └── trading-server.service
│
└── docs/
    ├── getting-started.md
    ├── api-reference.md
    ├── architecture.md
    └── examples/
```

---

## 🎯 Resumen de Decisiones Finales
```
┌─────────────────────────────────────────────────────────────────┐
│                    ARQUITECTURA FINAL                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│ ✅ Rust Edition: 2024 (sin mod.rs)                              │
│ ✅ GUI Framework: GTK4 + Relm4                                   │
│ ✅ Comunicación: Solo gRPC (Tonic)                               │
│ ✅ Base de datos: SQLite → PostgreSQL (producción)              │
│ ✅ Backtest engine: Polars (primario)                            │
│ ✅ Estrategias runtime: Rhai                                     │
│ ✅ Multi-timeframe: Built-in                                     │
│ ✅ Modo: Solo Cliente-Servidor (no local)                        │
│                                                                  │
│ Estructura modular: 15 crates independientes                    │
│ Separación clara: Proto → Server → Client → GUI/CLI             │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘