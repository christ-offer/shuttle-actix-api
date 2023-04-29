use actix_files::NamedFile;
//use actix_web::{get, web::ServiceConfig, Responder};
use actix_web::{ web::{self, ServiceConfig}, Responder, };
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_static_folder::StaticFolder;
use std::path::PathBuf;
use shuttle_service::ResourceBuilder

mod api;
mod models;

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html")
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))
}


#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(web::resource("/").route(web::get().to(index)));
        cfg.service(api::openmeteo::openmeteo);
    };

    Ok(config.into())
}

