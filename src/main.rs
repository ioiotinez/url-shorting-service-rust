mod db;
mod models;
mod routes;

use actix_web::{web, App, HttpServer};
use db::create_pool;
use models::AppState;
use routes::{health_check, index_delete, index_post, index_put, index_shorten, index_stats};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = create_pool().await;

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
