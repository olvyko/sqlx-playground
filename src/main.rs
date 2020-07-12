mod models;

use async_trait::async_trait;
use models::*;
use sqlx::PgPool;

pub type Error = Box<dyn std::error::Error + Sync + Send>;
pub type Result<T> = std::result::Result<T, Error>;

#[async_trait]
pub trait ProvideAuthn {
    async fn get_user_by_username(&self, username: &str) -> Result<Option<UserEntity>>;
}

#[async_trait]
impl ProvideAuthn for PgPool {
    async fn get_user_by_username(&self, username: &str) -> Result<Option<UserEntity>> {
        let user = sqlx::query_as_unchecked!(
            UserEntity,
            r#"
                SELECT id, username, preferences, created_at
                FROM users
                WHERE username=$1
            "#,
            username
        )
        .fetch_optional(self)
        .await?;
        Ok(user)
    }
}

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
    let _user = get_user_by_username(&pool, "admin").await?;
    let user = pool.get_user_by_username("admin").await?;
    println!("{:?}", user);
    Ok(())
}
