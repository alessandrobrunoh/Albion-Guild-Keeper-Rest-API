use serde::{Deserialize, Serialize};

// Definisce una struttura chiamata Discord che contiene informazioni su un utente Discord.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64, 
    pub server_name: String, 
    pub username: String,
    pub joined_at: String,
    pub discord_id: i64,
}