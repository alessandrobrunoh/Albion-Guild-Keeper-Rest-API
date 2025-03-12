use actix_web::{web::Query, HttpResponse};

use crate::models::auth::auth::Auth;

pub async fn login(query: Query<Auth>) -> HttpResponse {
    // Poiché i campi `token_type` e `token` sono privati, dobbiamo implementare metodi pubblici in `AuthDiscord` per ottenere questi valori.
    let token_type = &query.token_type; // Accesso diretto ai campi privati
    let token = &query.access_token; // Accesso diretto ai campi privati

    // Logica per gestire i parametri
    // Ad esempio, puoi restituire i parametri ricevuti come JSON
    HttpResponse::Ok().json(format!("Token Type: {} Token: {}", token_type, token))
}