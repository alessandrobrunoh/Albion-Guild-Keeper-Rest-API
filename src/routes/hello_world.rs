use actix_web::get;
use crate::controllers::hello_world as controller;

#[get("/")]
async fn hello_world() -> &'static str {
    controller::hello_world().await
}
