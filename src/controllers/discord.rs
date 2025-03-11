use crate::database::connect as database;
use crate::models::discords::Discords;
use crate::utils::surreal_int::SurrealInt;


pub async fn join_discord(
    discord_id: SurrealInt,
    discord_name: String,
    joined_at: String,
    balance: SurrealInt,
) -> Result<Option<Discords>, Box<dyn std::error::Error>> {

    let db = database::connect().await.unwrap(); // Connetti al database
    // Check if the discord_id already exists
    let existing_discord: Option<Discords> = db.select(("discords", discord_id.to_string())).await?;

    match existing_discord {
        Some(mut existing) => {
            // Discord ID exists, update only the discord_name
            existing.discord_name = discord_name;
            let updated_discord: Option<Discords> = db.update(("discords", discord_id.to_string())).content(existing).await?;
            Ok(updated_discord)
        }
        None => {
            // Discord ID does not exist, create a new record
            let discord = Discords {
                discord_id: discord_id,
                discord_name: discord_name,
                joined_at: joined_at,
                balance: balance,
            };

            let created_discord: Option<Discords> = db.create(("discords", discord_id.to_string()))
                .content(discord)
                .await?;
            Ok(created_discord)
        }
    }
}