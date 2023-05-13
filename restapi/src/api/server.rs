use crate::{
    types::structs::{ AppState, ApiParams, Character },
    api::lib::{ apply_filters, apply_sort },
};

use actix_web::{
    web,
    HttpResponse,
    error::{ QueryPayloadError, InternalError },
    HttpRequest,
    Error,
};
use serde_json::{ json };

pub async fn index(
    state: web::Data<AppState>,
    web::Query(params): web::Query<ApiParams>
) -> HttpResponse {
    let characters: &Vec<Character> = &*state.characters.read().unwrap();

    println!("{}", params);
    let filter_characters = apply_filters(params.clone(), characters);
    let mut sort_characters = apply_sort(params.clone(), filter_characters);
    if let Some(num) = &params.num {
        sort_characters = sort_characters.iter().copied().take(*num).collect::<Vec<_>>();
    }
    HttpResponse::Ok().content_type("application/json").json(sort_characters)
}

pub async fn fallback() -> HttpResponse {
    HttpResponse::NotFound().json(json!({ "error" : "404 Not Found" }))
}

pub fn query_error_handler(err: QueryPayloadError, _: &HttpRequest) -> Error {
    let message: String = err.to_string();
    return InternalError::from_response(
        err,
        HttpResponse::BadRequest().json(json!({ "error" : message }))
    ).into();
}