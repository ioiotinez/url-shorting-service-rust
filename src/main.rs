use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::FromRow;
use sqlx::MySqlPool;
use std::env;
use uuid::Uuid;

// Estado de la aplic.ación
struct AppState {
    db_pool: MySqlPool,
}

#[derive(serde::Serialize, serde::Deserialize, FromRow)]
struct ShortUrl {
    id: i32,
    original_url: String,
    short_code: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Servicio funcionando correctamente")
}

/*
Este metodo va a ser el encargado de crear nuevos shorten links.
Va a recibir dentro del body de la llamada una url.
Va a devolver un json con la url original y un codigo short. Tambien va a retornar un Id y la fecha de creacion y actualizacion
*/
#[post("/shorten")]
async fn index_post(data: web::Data<AppState>, body: web::Json<ShortUrl>) -> impl Responder {
    // Extract the ShortUrl from the request body
    let short_url = body.into_inner();

    // Insert into db
    let result = sqlx::query!(
        "INSERT INTO urls (original_url, short_code) VALUES (?, ?)",
        short_url.original_url,
        uuid::Uuid::new_v4().to_string()
    )
    .execute(&data.db_pool)
    .await
    .expect("Error al insertar el short url");

    HttpResponse::Created().json(ShortUrl {
        id: result.last_insert_id() as i32,
        original_url: short_url.original_url.clone(),
        short_code: short_url.short_code.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    })
}

/*
Este metodo va a ser el encargado de actualizar un shorten link.
Va a recibir un parametro en la url que va a ser el short code.
Va a recibir dentro del body de la llamada una url.
Va a devolver un json con la url original y un codigo short. Tambien va a retornar un Id y la fecha de creacion y actualizacion
*/
#[put("/shorten/{short}")]
async fn index_put(short: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("Short code: {}", short.into_inner()))
}

/*
Este metodo va a ser el encargado de redirigir a la url original.
Va a recibir un parametro en la url que va a ser el short code.
Va a devolver un redirect a la url original.
*/
#[get("/shorten/{short}")]
async fn index_shorten(short: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let short_code = short.into_inner();

    let short = sqlx::query!(
       "SELECT id, original_url, short_code, created_at, updated_at FROM urls WHERE short_code = ?",
       short_code
   )
    .fetch_one(&data.db_pool)
    .await
    .expect("Error al buscar el short code");

    // Redirect to original URL from short
    let original_url = short.original_url.clone();

    HttpResponse::Found()
        .append_header(("Location", original_url))
        .finish()
}

/*
Este metodo va a ser el encargado de eliminar un shorten link.
Va a recibir un parametro en la url que va a ser el short code.*/
#[delete("/shorten/{short}")]
async fn index_delete() -> impl Responder {
    "Hello world!"
}

#[get("/shorten/{short}/stats")]
async fn index_stats() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL debe estar configurada en .env");

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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db_pool: pool.clone(),
            }))
            .service(health_check)
            .service(index_shorten)
            .service(index_delete)
            .service(index_stats)
            .service(index_put)
            .service(index_post)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
