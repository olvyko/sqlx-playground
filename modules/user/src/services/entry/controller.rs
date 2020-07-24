use super::traits::*;
use async_trait::async_trait;
use shared::*;
use sqlx::Row;

pub struct EntryController;

#[async_trait]
impl EntryDb for EntryController {
    async fn create_customer(&self, conn: &mut PgConnection, username: &str) -> Result<CustomerComponent> {
        let component = CustomerComponent {
            id: new_uuid(),
            username: username.to_owned(),
            customer_type: CustomerTypeComponent::User,
            preferences: Json(PreferencesComponent {
                online: true,
                sounds: true,
            }),
            created_at: now_unix_time(),
        };
        sqlx::query(
            r#"
            INSERT INTO customer(id, username, customer_type, preferences, created_at)
            VALUES($1, $2, $3, $4, $5)
        "#,
        )
        .bind(component.id)
        .bind(component.username.clone())
        .bind(component.customer_type as i16)
        .bind(serde_json::to_value(&component.preferences)?)
        .bind(component.created_at)
        .execute(conn)
        .await?;
        Ok(component)
    }

    async fn get_customer_by_username(&self, conn: &mut PgConnection, username: &str) -> Result<Option<CustomerComponent>> {
        let component = sqlx::query_as(
            r#"
            SELECT
                id, username, customer_type, preferences, created_at
            FROM customer
            WHERE username=$1
        "#,
        )
        .bind(username)
        .fetch_optional(conn)
        .await?;
        Ok(component)
    }

    async fn get_customer_by_email(&self, conn: &mut PgConnection, email: &str) -> Result<Option<CustomerComponent>> {
        let component = sqlx::query_as(
            r#"
            SELECT
                c.id, c.username, c.customer_type, c.preferences, c.created_at
            FROM customer AS c
                LEFT JOIN email AS e ON e.id=(SELECT id FROM email WHERE customer_id=c.id ORDER BY created_at DESC LIMIT 1)
            WHERE e.email=$1
        "#,
        )
        .bind(email)
        .fetch_optional(conn)
        .await?;
        Ok(component)
    }

    async fn get_customer_with_email_by_username(
        &self,
        conn: &mut PgConnection,
        username: &str,
    ) -> Result<Option<(CustomerComponent, Option<EmailComponent>)>> {
        let components = sqlx::query(
            r#"
            SELECT
                c.id, c.username, c.customer_type, c.preferences, c.created_at,
                e.id, e.customer_id, e.email, e.created_at
            FROM customer AS c
                LEFT JOIN email AS e ON e.id=(SELECT id FROM email WHERE customer_id=c.id ORDER BY created_at DESC LIMIT 1)
            WHERE c.username=$1
        "#,
        )
        .bind(username)
        .try_map(|row: sqlx::postgres::PgRow| {
            let user = CustomerComponent {
                id: row.try_get(0)?,
                username: row.try_get(1)?,
                customer_type: row.try_get(2)?,
                preferences: row.try_get(3)?,
                created_at: row.try_get(4)?,
            };
            let email = match row.try_get::<Option<Uuid>, _>(5)?.is_some() {
                true => Some(EmailComponent {
                    id: row.try_get(5)?,
                    customer_id: row.try_get(6)?,
                    email: row.try_get(7)?,
                    created_at: row.try_get(8)?,
                }),
                false => None,
            };
            Ok((user, email))
        })
        .fetch_optional(conn)
        .await?;
        Ok(components)
    }

    async fn create_email(&self, conn: &mut PgConnection, email: &str, customer_id: Uuid) -> Result<EmailComponent> {
        let component = EmailComponent {
            id: new_uuid(),
            customer_id,
            email: email.to_owned(),
            created_at: now_unix_time(),
        };
        sqlx::query(
            r#"
            INSERT INTO email(id, customer_id, email, created_at)
            VALUES($1, $2, $3, $4)
        "#,
        )
        .bind(component.id)
        .bind(component.customer_id)
        .bind(component.email.clone())
        .bind(component.created_at)
        .execute(conn)
        .await?;
        Ok(component)
    }
}

#[allow(unused)]
impl EntryMailer for EntryController {
    fn send_verification_mail(&self, to: &str, email_verify_token: &str) -> Result<()> {
        Ok(())
    }
}
