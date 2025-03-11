use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use shuttle_actix_web::ShuttleActixWeb;
use serde_json::json;

mod controllers;
mod database;
mod routes;
mod models;
mod utils;

use crate::routes::{
    hello_world::hello_world,
    auth_discord::auth_discord,
    discord::join_discord,
    test::test,
    user::join_user
};

#[get("/")]
async fn admin_check() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({ "is_admin": false }))
}

#[get("/create_user")]
async fn create_user() -> &'static str {
    "User created!"
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut web::ServiceConfig| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        cfg.service(
            web::scope("/api/v1")
                .wrap(cors)
                .service(admin_check)
                .service(hello_world)
                .service(create_user)
                .service(auth_discord)
                .service(join_discord)
                .service(test)
                .service(join_user)
        );
    };

    println!("Starting server at http://localhost:8000");
    Ok(config.into())
}