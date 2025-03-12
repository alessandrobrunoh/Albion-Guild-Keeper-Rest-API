use actix_web::{web::Query, HttpResponse};

use crate::models::auth::auth::Auth;

pub async fn auth() -> HttpResponse {
    HttpResponse::BadRequest().json("true")
}   