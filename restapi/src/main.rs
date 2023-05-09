mod api;
mod types;
mod readfile;
use actix_web::{ web::{ self, QueryConfig }, App, HttpServer };
use api::server;
use std::env;
use types::structs::AppState;
use readfile::get_content;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let port = env::var("PORT").expect("not PORT set in ENV").parse::<u16>().expect("PORT ENV not a number");

    let httpserver = HttpServer::new(|| {
        let file_name: String = env::var("JSON_FILE").expect("not JSON_FILE set in ENV");
        
        App::new()
            .app_data(QueryConfig::default().error_handler(server::query_error_handler))
            .app_data(web::Data::new(AppState { characters: get_content( file_name ) }))
            .service(web::scope("/api/v1").service(server::index))
            .default_service(web::to(server::fallback))
    }).bind(("127.0.0.1", port))?;

    let server_addr: Vec<std::net::SocketAddr> = httpserver.addrs();
    
    println!("\tRunning server at http://{:?}/",server_addr.first().unwrap());
    httpserver.run().await
}