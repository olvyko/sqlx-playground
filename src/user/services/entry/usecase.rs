use crate::shared::*;
use async_trait::async_trait;
use sqlx::PgConnection;

#[async_trait]
pub trait EntryUsecase {
    async fn create_customer(&mut self, username: &str) -> Result<CustomerEntity>;
    async fn get_customer_by_username(&mut self, username: &str) -> Result<Option<CustomerEntity>>;
    async fn get_customer_by_email(&mut self, email: &str) -> Result<Option<CustomerEntity>>;
    async fn get_customer_with_email_by_username(
        &mut self,
        username: &str,
    ) -> Result<Option<(CustomerEntity, EmailEntity)>>;
    async fn create_email(&mut self, email: &str, user_id: Uuid) -> Result<EmailEntity>;
}

#[async_trait]
impl EntryUsecase for PgConnection {
    async fn create_customer(&mut self, username: &str) -> Result<CustomerEntity> {
        let entity = CustomerEntity {
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
                INSERT INTO customer(id, username, preferences, created_at)
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

    async fn get_customer_by_username(&mut self, username: &str) -> Result<Option<CustomerEntity>> {
        let user = sqlx::query_as_unchecked!(
            CustomerEntity,
            r#"
                SELECT
                    id, username, preferences, created_at
                FROM customer
                WHERE username=$1
            "#,
            username
        )
        .fetch_optional(self)
        .await?;
        Ok(user)
    }

    async fn get_customer_by_email(&mut self, email: &str) -> Result<Option<CustomerEntity>> {
        let user = sqlx::query_as_unchecked!(
            CustomerEntity,
            r#"
                SELECT
                    c.id, c.username, c.preferences, c.created_at
                FROM customer AS c
                    LEFT JOIN email AS e ON e.id=(SELECT id FROM email WHERE customer_id=c.id ORDER BY created_at DESC LIMIT 1)
                WHERE e.email=$1
            "#,
            email
        )
        .fetch_optional(self)
        .await?;
        Ok(user)
    }

    async fn get_customer_with_email_by_username(
        &mut self,
        username: &str,
    ) -> Result<Option<(CustomerEntity, EmailEntity)>> {
        let row = sqlx::query!(
            r#"
                SELECT
                    c.id AS customer_id, c.username, c.preferences, c.created_at AS customer_created_at,
                    e.id AS email_id, e.customer_id AS email_customer_id, e.email, e.created_at AS email_created_at
                FROM customer AS c
                    LEFT JOIN email AS e ON e.id=(SELECT id FROM email WHERE customer_id=c.id ORDER BY created_at DESC LIMIT 1)
                WHERE c.username=$1
            "#,
            username
        )
        .fetch_optional(self)
        .await?;

        row.map(|row| {
            let user = CustomerEntity {
                id: row.customer_id,
                username: row.username,
                preferences: serde_json::from_value(row.preferences)?,
                created_at: row.customer_created_at,
            };
            let email = EmailEntity {
                id: row.email_id,
                customer_id: row.email_customer_id,
                email: row.email,
                created_at: row.email_created_at,
            };
            Ok((user, email))
        })
        .transpose()
    }

    async fn create_email(&mut self, email: &str, customer_id: Uuid) -> Result<EmailEntity> {
        let entity = EmailEntity {
            id: new_uuid(),
            customer_id,
            email: email.to_owned(),
            created_at: now_unix_time(),
        };
        sqlx::query!(
            r#"
                INSERT INTO email(id, customer_id, email, created_at)
                VALUES($1, $2, $3, $4)
            "#,
            entity.id,
            entity.customer_id,
            entity.email,
            entity.created_at
        )
        .execute(self)
        .await?;
        Ok(entity)
    }
}
