use actix_web::{delete, get, post, put, web, App, HttpServer, Responder};

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
async fn index_post() -> impl Responder {
    "Hello world!"
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
    "Hello world!"
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

    HttpServer::new(||
        App::new()
            .service(index))
            .bind("127.0.0.1:8080")?
            .run()
            .await
}
