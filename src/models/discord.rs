use serde::{Deserialize, Serialize};

use crate::utils::surreal_int::SurrealInt;

// Definisce una struttura chiamata Discord che contiene informazioni su un utente Discord.
#[derive(Debug, Serialize, Deserialize)]
pub struct Discord {
    pub id: SurrealInt, 
    pub discord_name: String, 
    pub joined_at: String
}