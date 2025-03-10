use actix_web::{web::Query, HttpResponse};

use crate::models::authentication::AuthDiscord;
pub async fn auth_discord(query: Query<AuthDiscord>) -> HttpResponse {
    // Poiché i campi `token_type` e `token` sono privati, dobbiamo implementare metodi pubblici in `AuthDiscord` per ottenere questi valori.
    let token_type = &query.token_type; // Accesso diretto ai campi privati
    let token = &query.token; // Accesso diretto ai campi privati

    // Logica per gestire i parametri
    // Ad esempio, puoi restituire i parametri ricevuti come JSON
    HttpResponse::Ok().json(format!("Token Type: {} Token: {}", token_type, token))
}