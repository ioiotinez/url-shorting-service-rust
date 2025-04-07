use crate::models::{AppState, ShortUrl, ShortUrlRequest};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Servicio funcionando correctamente")
}

#[post("/shorten")]
pub async fn index_post(
    data: web::Data<AppState>,
    body: web::Json<ShortUrlRequest>,
) -> impl Responder {
    let short_url_request = body.into_inner();
    let short_code = Uuid::new_v4().to_string()[..4].to_string();

    let result = sqlx::query!(
        "INSERT INTO urls (original_url, short_code) VALUES (?, ?)",
        short_url_request.url,
        short_code
    )
    .execute(&data.db_pool)
    .await
    .expect("Error al insertar el short url");

    HttpResponse::Created().json(ShortUrl {
        id: result.last_insert_id() as i32,
        original_url: short_url_request.url,
        short_code,
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
        access_count: 0,
    })
}

#[put("/shorten/{short}")]
pub async fn index_put(
    short: web::Path<String>,
    body: web::Json<ShortUrlRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let short_code = short.into_inner();
    let short_url_request = body.into_inner();

    let result = sqlx::query!(
        "UPDATE urls SET original_url = ? WHERE short_code = ?",
        short_url_request.url,
        short_code
    )
    .execute(&data.db_pool)
    .await
    .expect("Error al actualizar el short url");

    if result.rows_affected() > 0 {
        let result = sqlx::query_as!(
            ShortUrl,
            "SELECT id, original_url, short_code, created_at, updated_at, access_count FROM urls WHERE short_code = ?",
            short_code
        )
        .fetch_one(&data.db_pool)
        .await
        .expect("Error al buscar el short code");

        HttpResponse::Ok().json(result)
    } else {
        HttpResponse::NotFound().body(format!("Short code {} no encontrado", short_code))
    }
}

#[get("/shorten/{short}")]
pub async fn index_shorten(short: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let short_code = short.into_inner();

    let short = sqlx::query!(
       "SELECT id, original_url, short_code, created_at, updated_at FROM urls WHERE short_code = ?",
       short_code
   )
    .fetch_one(&data.db_pool)
    .await
    .expect("Error al buscar el short code");

    sqlx::query!(
        "UPDATE urls SET access_count = access_count + 1 WHERE short_code = ?",
        short_code
    )
    .execute(&data.db_pool)
    .await
    .expect("Error al actualizar el access count");

    let original_url = short.original_url.clone();

    HttpResponse::Found()
        .append_header(("Location", original_url))
        .finish()
}

#[delete("/shorten/{short}")]
pub async fn index_delete(short: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let short_code = short.into_inner();

    let result = sqlx::query!("DELETE FROM urls WHERE short_code = ?", short_code)
        .execute(&data.db_pool)
        .await
        .expect("Error al eliminar el short url");

    if result.rows_affected() > 0 {
        HttpResponse::Ok().body(format!("Short code {} eliminado", short_code))
    } else {
        HttpResponse::NotFound().body(format!("Short code {} no encontrado", short_code))
    }
}

#[get("/shorten/{short}/stats")]
pub async fn index_stats(short: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let short_code = short.into_inner();

    let result = sqlx::query_as!(
        ShortUrl,
        "SELECT id, original_url, short_code, created_at, updated_at, access_count FROM urls WHERE short_code = ?",
        short_code
    )
    .fetch_one(&data.db_pool)
    .await
    .expect("Error al buscar el short code");

    HttpResponse::Ok().json(result)
}
