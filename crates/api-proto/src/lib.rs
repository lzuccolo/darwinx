//! DarwinX gRPC Protocol Buffers
//!
//! Este crate contiene las definiciones de Protocol Buffers y código generado
//! para la comunicación gRPC entre cliente y servidor.

// Nota: Los archivos son generados por build.rs en compile time
// Si ves errores, ejecuta: cargo clean && cargo build

#![allow(clippy::all)]
#![allow(warnings)]

// Incluir archivos generados directamente
#[path = "generated/darwinx.common.rs"]
pub mod common;

#[path = "generated/darwinx.strategy.rs"]
pub mod strategy;

#[path = "generated/darwinx.backtest.rs"]
pub mod backtest;

#[path = "generated/darwinx.optimizer.rs"]
pub mod optimizer;

#[path = "generated/darwinx.live.rs"]
pub mod live;

// Re-exports de tipos comunes para facilitar el uso
pub use common::*;

// Servicios gRPC organizados
pub mod services {
    pub use crate::strategy::{
        strategy_service_client::StrategyServiceClient,
        strategy_service_server::{StrategyService, StrategyServiceServer},
    };
    
    pub use crate::backtest::{
        backtest_service_client::BacktestServiceClient,
        backtest_service_server::{BacktestService, BacktestServiceServer},
    };
    
    pub use crate::optimizer::{
        optimizer_service_client::OptimizerServiceClient,
        optimizer_service_server::{OptimizerService, OptimizerServiceServer},
    };
    
    pub use crate::live::{
        live_trading_service_client::LiveTradingServiceClient,
        live_trading_service_server::{LiveTradingService, LiveTradingServiceServer},
    };
}