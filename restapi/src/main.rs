mod api;
use actix_web::{App, HttpServer};
use api::server::index;
const PORT: u16 = 1000;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("running server at http://localhost:{PORT}/");
    HttpServer::new(|| {
        App::new()
            .service(index)

    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
