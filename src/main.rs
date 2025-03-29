use actix_web::{delete, get, post, put, web, App, HttpServer, Responder};
use mysql_async::Pool;

#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
}

/*
Este metodo va a ser el encargado de crear nuevos shorten links.
Va a recibir dentro del body de la llamada una url.
Va a devolver un json con la url original y un codigo short. Tambien va a retornar un Id y la fecha de creacion y actualizacion
*/
#[post("/shorten")]
async fn index_post(pool: web::Data<Pool>) -> impl Responder {
    conn = pool.get_conn().await.unwrap();

    // Obtener el body de la llamada
    let body = web::Json::<String>::from_request(&req, &mut payload).await.unwrap();
    let url = body.into_inner();

    // Validar la url
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return HttpResponse::BadRequest().body("La url no es valida");
    }
    // Crear el short code
    let short_code = uuid::Uuid::new_v4().to_string();
    // Guardar la url y el short code en la base de datos
    let query = format!("INSERT INTO urls (url, short_code) VALUES ('{}', '{}')", url, short_code);
    conn.query_drop(query).await.unwrap();

    HttpResponse::Ok().json(json!({
        "url": url,
        "short_code": short_code,
        "id": 1,
        "created_at": "2023-01-01 00:00:00",
        "updated_at": "2023-01-01 00:00:00"
    }))
    
}

/*
Este metodo va a ser el encargado de actualizar un shorten link.
Va a recibir un parametro en la url que va a ser el short code.
Va a recibir dentro del body de la llamada una url.
Va a devolver un json con la url original y un codigo short. Tambien va a retornar un Id y la fecha de creacion y actualizacion
*/
#[put("/shorten/{short}")]
async fn index_put() -> impl Responder {
    "Hello world!"
}

/*
Este metodo va a ser el encargado de redirigir a la url original.
Va a recibir un parametro en la url que va a ser el short code.
Va a devolver un redirect a la url original.
*/
#[get("/shorten/{short}")]
async fn index_shorten() -> impl Responder {
    // Obtener el short code de la url
    let short_code = web::Path::<String>::from_request(&req, &mut payload).await.unwrap();
    // Obtener la url original de la base de datos
    let query = format!("SELECT url FROM urls WHERE short_code = '{}'", short_code);
    let result = conn.query_first(query).await.unwrap();
    // Si no se encuentra la url original, devolver un error 404
    if result.is_none() {
        return HttpResponse::NotFound().body("No se encontro la url");
    }
    let url = result.unwrap();

    // Redirigir a la url original
    HttpResponse::Found()
        .header("Location", url)
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
async fn main() -> std::io::Result<()>{
    dotenv::dotenv().ok();
    mysql::init_pool().await.unwrap();
    let pool = mysql::get_pool().await.unwrap();
    
    let pool_data = web::Data::new(pool);

    HttpServer::new(||
        App::new()
            .app_data(pool_data.clone())
            .service(index_shorten)
            .service(index_delete)
            .service(index_stats)
            .service(index_put)
            .service(index_post)
            .service(index))
            .bind("127.0.0.1:8080")?
            .run()
            .await
}
