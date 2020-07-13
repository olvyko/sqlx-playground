mod shared;

use async_trait::async_trait;
use shared::*;
use sqlx::types::Json;
use sqlx::{PgConnection, PgPool};

#[async_trait]
pub trait Truncate {
    async fn truncate_all_tables(&mut self) -> Result<()>;
}

#[async_trait]
impl Truncate for PgConnection {
    async fn truncate_all_tables(&mut self) -> Result<()> {
        sqlx::query!("TRUNCATE TABLE users, emails").execute(self).await?;
        Ok(())
    }
}

#[async_trait]
pub trait ProvideAuthn {
    async fn create_user(&mut self, username: &str) -> Result<UserEntity>;
    async fn get_user_by_username(&mut self, username: &str) -> Result<Option<UserEntity>>;
    async fn get_user_with_email_by_username(&mut self, username: &str) -> Result<Option<(UserEntity, EmailEntity)>>;
    async fn create_email(&mut self, email: &str, user_id: Uuid) -> Result<EmailEntity>;
}

#[async_trait]
impl ProvideAuthn for PgConnection {
    async fn create_user(&mut self, username: &str) -> Result<UserEntity> {
        let entity = UserEntity {
            id: new_uuid(),
            username: username.to_owned(),
            preferences: Json(PreferencesEntity {
                online: true,
                sounds: true,
            }),
            created_at: now_unix_time(),
        };
        sqlx::query!(
            r#"
                INSERT INTO users(id, username, preferences, created_at)
                VALUES($1, $2, $3, $4)
            "#,
            entity.id,
            entity.username,
            serde_json::to_value(&entity.preferences)?,
            entity.created_at
        )
        .execute(self)
        .await?;
        Ok(entity)
    }

    async fn get_user_by_username(&mut self, username: &str) -> Result<Option<UserEntity>> {
        let user = sqlx::query_as_unchecked!(
            UserEntity,
            r#"
                SELECT
                    id, username, preferences, created_at
                FROM users
                WHERE username=$1
            "#,
            username
        )
        .fetch_optional(self)
        .await?;
        Ok(user)
    }

    async fn get_user_with_email_by_username(&mut self, username: &str) -> Result<Option<(UserEntity, EmailEntity)>> {
        let row = sqlx::query!(
            r#"
                SELECT
                    u.id AS user_id, u.username, u.preferences, u.created_at AS user_created_at,
                    e.id AS email_id, e.user_id AS email_user_id, e.email, e.created_at AS email_created_at
                FROM users AS u
                    LEFT JOIN emails AS e ON e.id=(SELECT id FROM emails WHERE user_id=u.id ORDER BY created_at DESC LIMIT 1)
                WHERE u.username=$1
            "#,
            username
        )
        .fetch_optional(self)
        .await?;

        row.map(|row| {
            let user = UserEntity {
                id: row.user_id,
                username: row.username,
                preferences: serde_json::from_value(row.preferences)?,
                created_at: row.user_created_at,
            };
            let email = EmailEntity {
                id: row.email_id,
                user_id: row.email_user_id,
                email: row.email,
                created_at: row.email_created_at,
            };
            Ok((user, email))
        })
        .transpose()
    }

    async fn create_email(&mut self, email: &str, user_id: Uuid) -> Result<EmailEntity> {
        let entity = EmailEntity {
            id: new_uuid(),
            user_id,
            email: email.to_owned(),
            created_at: now_unix_time(),
        };
        sqlx::query!(
            r#"
                INSERT INTO emails(id, user_id, email, created_at)
                VALUES($1, $2, $3, $4)
            "#,
            entity.id,
            entity.user_id,
            entity.email,
            entity.created_at
        )
        .execute(self)
        .await?;
        Ok(entity)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let url = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL is not set in .env file")?;
    let pool = PgPool::new(&url).await?;
    let mut conn = pool.acquire().await?;
    conn.truncate_all_tables().await?;
    {
        let mut tx = pool.begin().await?;
        let _created_user = tx.create_user("admin").await?;
    }
    let user = conn.get_user_by_username("admin").await?;
    assert!(user.is_none());
    {
        let mut tx = pool.begin().await?;
        let _created_user = tx.create_user("admin").await?;
        tx.commit().await?;
    }
    let user = conn.get_user_by_username("admin").await?;
    assert!(user.is_some());
    Ok(())
}
