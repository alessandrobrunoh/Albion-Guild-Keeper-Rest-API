use crate::database::connect as database;
use actix_web::HttpResponse;
use crate::models::discord::Discord;

pub async fn test() -> HttpResponse {
    let db = database::connect().await.unwrap(); // Connetti al database
    
    // Seleziona tutti i record dalla tabella `discords`
    let result: Result<Vec<Discord>, surrealdb::Error> = db.select("discords").await;

    // Gestisci il risultato
    match result {
        Ok(data) => HttpResponse::Ok().json(data), // Restituisci i dati trovati
        Err(_) => HttpResponse::InternalServerError().finish(), // Restituisci un errore 500 in caso di errore
    }
}