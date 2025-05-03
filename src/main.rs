mod models;
mod utils;
mod db;
mod rest;
//use diesel::prelude::*;
//use std::time::SystemTime;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
            .route("/create_list", web::post().to(rest::create_list))
            .route("/create_item", web::post().to(rest::create_item))
            .route("/delete_item", web::post().to(rest::delete_item))
            .route("/delete_multi_item", web::post().to(rest::delete_multiple_items))
            .route("/complete_item/{item_id}", web::post().to(rest::complete_item))
            .route("/complete_list/{list_id}", web::post().to(rest::complete_list))
            .route("/get_list/{list_id}", web::get().to(rest::query_at_node))
            .route("/update_items", web::post().to(rest::bulk_update))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

