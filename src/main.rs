use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

mod controllers;
mod database;
mod routes;
mod models;
mod utils;

use crate::routes::hello_world::hello_world;
use crate::routes::auth_discord::auth_discord;
use crate::routes::test::test;

#[get("/create_user")]
async fn create_user() -> &'static str {
    "User created!"
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(create_user);
        cfg.service(auth_discord);
        cfg.service(test);
    };

    Ok(config.into())
}