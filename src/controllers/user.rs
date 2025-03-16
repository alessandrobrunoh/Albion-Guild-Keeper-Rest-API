use crate::database::connect as database;
use crate::models::user::User;
use crate::utils::surreal_int::SurrealInt;

pub async fn join_user(
    id: SurrealInt,
    username: String,
    server_name: String,
    joined_at: String,
    discord_id: SurrealInt,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    let db = database::connect().await.unwrap();
    {
        let query = format!(
            "CREATE users SET id = {}, server_name = '{}', username = '{}', joined_at = '{}'",
            id, server_name, username, joined_at
        );
        let created_user = db.query(query).await?; 

        match created_user.check() {
            Ok(_) => {
                let relation_query =
                    format!("RELATE discords:{}->joined->users:{}", discord_id, id);
                let _create_relation = db.query(relation_query).await?;

                Ok(Some(User {
                    id: id.into(),
                    username,
                    server_name,
                    joined_at,
                }))
            }
            Err(e) => {
                if e.to_string().contains("already exists") {
                    let query = format!(
                        "UPDATE users SET server_name = '{}', username = '{}' WHERE id = {}",
                        server_name, username, id
                    );
                    db.query(query).await?;
                    Ok(None)
                } else {
                    return Err(e.into());
                }
            }
        }
    }
}
