mod api;
mod types;
use actix_web::{ App, HttpServer };
use api::server::index;
use types::enums::Rarities;
const PORT: u16 = 1000;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("running server at http://localhost:{PORT}/");
    let a = Rarities::LR;
    println!("{}", a.as_string());
    HttpServer::new(|| { App::new().service(index) })
        .bind(("127.0.0.1", PORT))?
        .run().await
}