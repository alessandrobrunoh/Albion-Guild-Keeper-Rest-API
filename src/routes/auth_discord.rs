use actix_web::{get, HttpResponse, web::Query};
use crate::{
    controllers::auth_discord as controller,
    models::authentication::AuthDiscord
};

#[get("/auth/discord")]
async fn auth_discord(query: Query<AuthDiscord>) -> HttpResponse {
    controller::auth_discord(query).await
}