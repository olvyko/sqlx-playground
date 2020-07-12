use serde::{Deserialize, Serialize};
use sqlx::types::{chrono::NaiveDateTime, Json, Uuid};
use sqlx::PgPool;

pub type EntityId = Uuid;

#[derive(Debug)]
pub struct UserEntity {
    pub id: EntityId,
    pub username: String,
    pub preferences: Json<UserPreferences>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    pub profit: bool,
    pub wagered: bool,
    pub online: bool,
    pub sounds: bool,
}

pub type Error = Box<dyn std::error::Error + Sync + Send>;
pub type Result<T> = std::result::Result<T, Error>;

async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<Option<UserEntity>> {
    let user = sqlx::query_as_unchecked!(
        UserEntity,
        r#"
            SELECT id, username, preferences, created_at
            FROM users
            WHERE username=$1
        "#,
        username
    )
    .fetch_optional(pool)
    .await?;
    Ok(user)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let url = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL is not set in .env file")?;
    let pool = PgPool::new(&url).await?;
    let user = get_user_by_username(&pool, "admin").await?;
    println!("{:?}", user);
    Ok(())
}
