use actix_web::{get, HttpResponse, web::Query};
use crate::{
    controllers::auth::login as controller,
    models::auth::auth::Auth
};

#[get("/auth/login")]
async fn login(query: Query<Auth>) -> HttpResponse {
    controller::login(query).await
}