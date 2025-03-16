use actix_web::{put, HttpResponse};
use actix_web::web::{Json, Path};
use serde::Deserialize;
use crate::controllers::user as controllers;
use crate::utils::surreal_int::SurrealInt;

#[derive(Deserialize)]
struct UserInput {
    username: String,
    server_name: String,
    joined_at: String,
    discord_id: SurrealInt,
}

#[put("/user/{user_id}")]
pub async fn join_user(user_id: Path<SurrealInt>, user_data: Json<UserInput>) -> HttpResponse {
    let username = user_data.username.clone();
    let server_name = user_data.server_name.clone();
    let joined_at = user_data.joined_at.clone();
    let discord_id = user_data.discord_id.clone();
    
    let result = controllers::join_user(user_id.into_inner(), username, server_name, joined_at, discord_id).await;

    println!("{:#?}", result);

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::Ok().body("Record Updated"),    
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}