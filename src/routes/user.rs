use actix_web::{put, HttpResponse};
use actix_web::web::{Json, Path};
use crate::controllers::user as controllers;
use crate::models::user::User;

#[put("/user/{user_id}")]
pub async fn join_user(user_id: Path<i64>, user_data: Json<User>) -> HttpResponse {
    let user_id = user_id.into_inner();
    let user_data = user_data.into_inner();
    
    let result = controllers::join_user(user_id, user_data.username, user_data.server_name, user_data.joined_at, user_data.discord_id).await;

    println!("result: {:?}", result);

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::InternalServerError().body("error"),
        Err(_) => HttpResponse::InternalServerError().body("error"),
    }
}