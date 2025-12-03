//! Inicialización de base de datos

use sqlx::{Pool, Sqlite};
use std::path::{Path, PathBuf};

/// Inicializa la base de datos SQLite
pub async fn init_sqlite(path: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    // Resolver ruta (absoluta o relativa)
    let db_path = if Path::new(path).is_absolute() {
        PathBuf::from(path)
    } else {
        // Ruta relativa: resolver desde el directorio actual
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    };

    // Crear directorio padre si no existe
    if let Some(parent) = db_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .map_err(|e| {
                    let error_msg = format!(
                        "No se pudo crear el directorio '{}': {}. Verifica permisos de escritura.",
                        parent.display(), e
                    );
                    sqlx::Error::Io(std::io::Error::new(
                        std::io::ErrorKind::PermissionDenied,
                        error_msg
                    ))
                })?;
        }
        // Verificar permisos de escritura en el directorio
        if !parent.metadata()
            .map_err(|e| sqlx::Error::Io(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!("No se pudo acceder al directorio '{}': {}", parent.display(), e)
            )))?
            .permissions()
            .readonly() {
            // El directorio existe y tiene permisos de escritura
        }
    }

    // Convertir a string para la conexión
    let db_path_str = db_path.to_string_lossy().to_string();
    
    // Conectar a SQLite con opciones adicionales
    let connection_string = format!("sqlite:{}?mode=rwc", db_path_str);
    let pool = sqlx::SqlitePool::connect(&connection_string).await?;
    
    // Ejecutar migraciones
    // sqlx::migrate! busca las migraciones en el directorio relativo al crate
    // En tiempo de compilación, esto se resuelve a crates/strategy-store/migrations
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => {}
        Err(e) => {
            // Si las migraciones fallan, intentar con ruta absoluta desde el crate
            eprintln!("⚠️  Advertencia: Error al ejecutar migraciones: {}", e);
            eprintln!("   Intentando continuar...");
            // No fallar completamente, la base de datos puede existir ya
        }
    }
    
    Ok(pool)
}