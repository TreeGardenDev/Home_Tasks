use std::env;
use dotenvy::dotenv;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use actix_web::http::header::{HeaderMap, HeaderName};
use actix_web::{HttpRequest};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Need DATABASE_URL");
    let conn = PgConnection::establish(&db_url).unwrap();
    conn
}

pub async fn is_htmx(req: &HttpRequest) -> bool{
    let headers: &HeaderMap = req.headers();
    let mut is_hx_request = false;

    //look for Hx-Request header
    if let Some(hx_request) = headers.get(HeaderName::from_static("hx-request")) {
        if let Ok(hx_str) = hx_request.to_str() {
            println!("Hx-Request: {}", hx_str);
            is_hx_request=true;
        }
    }
    return is_hx_request;

}
