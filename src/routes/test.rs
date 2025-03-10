use actix_web::{get, HttpResponse};
use crate::controllers::test as controller;

#[get("/test")]
async fn test() -> HttpResponse {
    controller::test().await
}