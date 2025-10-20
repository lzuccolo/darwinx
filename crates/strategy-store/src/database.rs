//! Inicialización de base de datos

use sqlx::{Pool, Sqlite};
use std::path::Path;

/// Inicializa la base de datos SQLite
pub async fn init_sqlite(path: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    // Crear directorio si no existe
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent).ok();
    }

    // Conectar a SQLite
    let pool = sqlx::SqlitePool::connect(&format!("sqlite:{}", path)).await?;
    
    // Ejecutar migraciones
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}