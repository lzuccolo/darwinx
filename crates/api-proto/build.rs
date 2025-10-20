// build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/generated")
        .compile_protos(
            &[
                "proto/common.proto",
                "proto/strategy.proto",
                "proto/backtest.proto",
                "proto/optimizer.proto",
                "proto/live.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}