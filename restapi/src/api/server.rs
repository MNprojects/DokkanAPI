use crate::types::structs::{AppState, ApiParams};
use actix_web::{web, get, Responder, HttpResponse };
use serde_json::{json};

#[get("/")]
pub async fn index(state: web::Data<AppState>,web::Query(params): web::Query<ApiParams>) ->  impl Responder {
    println!("{}", params);
    HttpResponse::Ok().content_type("application/json").json(&*state.characters.read().unwrap())
}

pub async fn fallback() -> HttpResponse {
    HttpResponse::NotFound().json(json!({"error ":"404 Not Found"}))
}