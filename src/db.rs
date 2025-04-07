use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::env;

pub async fn create_pool() -> MySqlPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL debe estar configurada en .env");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error al crear el pool de conexiones");

    // Verificar la conexión
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .expect("Error al verificar la conexión a la base de datos");

    println!("✅ Conexión a MySQL establecida correctamente");
    pool
}
