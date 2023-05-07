use crate::types::structs::{ AppState, ApiParams };
use actix_web::{
    web,
    get,
    HttpResponse,
    error::{ QueryPayloadError, InternalError },
    HttpRequest,
    Error,
};
use serde_json::{ json };

#[get("/")]
pub async fn index(
    state: web::Data<AppState>,
    web::Query(params): web::Query<ApiParams>
) -> HttpResponse {
    let characters = &*state.characters.read().unwrap();
    println!("{}", params);

    HttpResponse::Ok().content_type("application/json").json(characters)
}

pub async fn fallback() -> HttpResponse {
    HttpResponse::NotFound().json(json!({ "error" : "404 Not Found" }))
}

pub fn query_error_handler(err: QueryPayloadError, _: &HttpRequest) -> Error {
    let message = err.to_string();
    return InternalError::from_response(
        err,
        HttpResponse::BadRequest().json(json!({ "error" : message }))
    ).into();
}