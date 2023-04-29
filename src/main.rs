use actix_files::NamedFile;
//use actix_web::{get, web::ServiceConfig};
use actix_web::{
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use shuttle_actix_web::ShuttleActixWeb;

mod api;
mod models;

#[get("/hello")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html")
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))
}

#[shuttle_runtime::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(web::resource("/").route(web::get().to(index)))
        cfg.service(api::openmeteo::openmeteo);
        cfg.service(hello_world);
    };

    Ok(config.into())
}