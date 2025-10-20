// build.rs
use std::path::PathBuf;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ruta al directorio de salida
    let out_dir = PathBuf::from("src/generated");
    
    // Crear directorio si no existe
    if !out_dir.exists() {
        std::fs::create_dir_all(&out_dir)?;
    }

    println!("cargo:warning=Generando archivos proto en {:?}", out_dir);

    // Configurar tonic-build
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(&out_dir)
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

    println!("cargo:warning=Archivos proto generados exitosamente");

    // Recompilar si los .proto cambian
    println!("cargo:rerun-if-changed=proto/common.proto");
    println!("cargo:rerun-if-changed=proto/strategy.proto");
    println!("cargo:rerun-if-changed=proto/backtest.proto");
    println!("cargo:rerun-if-changed=proto/optimizer.proto");
    println!("cargo:rerun-if-changed=proto/live.proto");
    
    Ok(())
}