use actix_web::{put, HttpResponse};
use actix_web::web::{Json, Path};
use crate::utils::surreal_int::SurrealInt;
use serde::Deserialize;
use crate::controllers::discord as controllers;


#[derive(Deserialize)]
struct DiscordServerInput {
    discord_name: String,
    joined_at: String,
    balance: SurrealInt,
}

#[put("/discord/{discord_id}")]
pub async fn join_discord(discord_id: Path<SurrealInt>, discord_data: Json<DiscordServerInput>) -> HttpResponse {
    let discord_name = discord_data.discord_name.clone();
    let joined_at = discord_data.joined_at.clone();
    let balance = discord_data.balance.clone();
    
    let result = controllers::join_discord(discord_id.into_inner(), discord_name, joined_at, balance).await;

    println!("result: {:?}", result);   

    match result {
        Ok(Some(_)) => HttpResponse::Ok().body("success"),
        Ok(None) => HttpResponse::InternalServerError().body("error"),    
        Err(_) => HttpResponse::InternalServerError().body("error"),
    }
}