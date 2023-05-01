use crate::types::structs::AppState;
use actix_web::{web, get, Responder, HttpResponse };

#[get("/")]
pub async fn index(state: web::Data<AppState>) ->  impl Responder {
    
    HttpResponse::Ok().content_type("application/json").json(&*state.characters.read().unwrap())
}