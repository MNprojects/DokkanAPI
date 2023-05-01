use actix_web::{ get, Responder, HttpResponse };
mod api {}
#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("test")
}