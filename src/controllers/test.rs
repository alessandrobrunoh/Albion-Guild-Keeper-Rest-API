use actix_web::HttpResponse;


pub async fn test() -> HttpResponse {

    HttpResponse::Ok().json(format!("Hello, World! TEST"))
}