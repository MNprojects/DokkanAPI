mod api;
mod types;
mod readfile;
use actix_web::{web, App, HttpServer };
use api::server::index;
use types::structs::AppState;
use readfile::get_content;


const PORT: u16 = 1000;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("running server at http://localhost:{PORT}/");


    HttpServer::new(|| { App::new().app_data(web::Data::new(AppState { characters : get_content() })).service(index) })
        .bind(("127.0.0.1", PORT))?
        .run().await
}