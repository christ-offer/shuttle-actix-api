use actix_files::NamedFile;
use actix_web::{ web::{self, ServiceConfig}, Responder };
use shuttle_actix_web::ShuttleActixWeb;
use std::path::PathBuf;

mod api;
mod models;
/*
async fn index(static_folder: PathBuf) -> impl Responder {
    NamedFile::open_async(static_folder"./static/index.html")
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))
}
 */

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let static_folder = static_folder.join("index.html");

    let index = NamedFile::open_async(static_folder)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))
    
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(web::resource("/").route(web::get().to(index)));
        cfg.service(api::openmeteo::openmeteo);
    };

    Ok(config.into())
}