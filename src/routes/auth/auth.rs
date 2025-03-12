use actix_web::{get, HttpResponse, web::Query};
use crate::{
    controllers::auth::auth as controller,
    models::auth::auth::Auth
};

#[get("/auth")]
async fn auth() -> HttpResponse {
    controller::auth().await
}