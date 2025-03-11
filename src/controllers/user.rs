use crate::database::connect as database;
use crate::models::user::User;

pub async fn join_user(
    id: i64,
    username: String,
    server_name: String,
    joined_at: String,
    discord_id: i64,
) -> Result<Option<User>, Box<dyn std::error::Error>> {

    let db = database::connect().await.unwrap(); // Connetti al database
    // Check if the user_id already exists
    let existing_user: Option<User> = db.select(("users", id.to_string())).await?;

    match existing_user {
        Some(mut existing) => {
            // user ID exists, update only the user_name
            existing.username = username;
            let updated_user: Option<User> = db.update(("users", id.to_string())).content(existing).await?;
            Ok(updated_user)
        }
        None => {
            // user ID does not exist, create a new record
            let user = User {
                id,
                server_name,
                username,
                joined_at,
                discord_id,
            };

            let created_user: Option<User> = db.create("users")
                .content(user)
                .await?;
            Ok(created_user)
        }
    }
}