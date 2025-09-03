mod models;
mod utils;
mod db;
mod rest;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest, Result};
use actix_web::http::header::{HeaderMap, HeaderName};
use actix_files::NamedFile;
use std::path::PathBuf;

async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./htmx/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}
async  fn viewlists(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./htmx/list.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}



//serve up htmx files on root
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/viewlists", web::get().to(viewlists))
            .route("/hey", web::get().to(manual_hello))
            .route("/create_list", web::post().to(rest::create_list))
            .route("/create_item", web::post().to(rest::create_item))
            .route("/delete_item", web::post().to(rest::delete_item))
            .route("/delete_multi_item", web::post().to(rest::delete_multiple_items))
            .route("/complete_item/{item_id}", web::post().to(rest::complete_item))
            .route("/complete_list/{list_id}", web::post().to(rest::complete_list))
            .route("/get_list/{list_id}", web::get().to(rest::query_at_node))
            .route("/update_items", web::post().to(rest::bulk_update))
            .route("/get_all_lists/{active_only}", web::get().to(rest::get_all_lists))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

