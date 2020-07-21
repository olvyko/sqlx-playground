use super::traits::*;
use async_trait::async_trait;
use shared::*;

pub struct EntryController;

#[async_trait]
impl EntryDb for EntryController {
    async fn create_customer(&self, conn: &mut PgConnection, username: &str) -> Result<CustomerComponent> {
        let component = CustomerComponent {
            id: new_uuid(),
            username: username.to_owned(),
            preferences: Json(PreferencesComponent {
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
            component.id,
            component.username,
            serde_json::to_value(&component.preferences)?,
            component.created_at
        )
        .execute(conn)
        .await?;
        Ok(component)
    }

    async fn get_customer_by_username(&self, conn: &mut PgConnection, username: &str) -> Result<Option<CustomerComponent>> {
        let component = sqlx::query_as_unchecked!(
            CustomerComponent,
            r#"
SELECT
    id, username, preferences, created_at
FROM customer
WHERE username=$1
"#,
            username
        )
        .fetch_optional(conn)
        .await?;
        Ok(component)
    }

    async fn get_customer_by_email(&self, conn: &mut PgConnection, email: &str) -> Result<Option<CustomerComponent>> {
        let component = sqlx::query_as_unchecked!(
            CustomerComponent,
            r#"
SELECT
    c.id, c.username, c.preferences, c.created_at
FROM customer AS c
    LEFT JOIN email AS e ON e.id=(SELECT id FROM email WHERE customer_id=c.id ORDER BY created_at DESC LIMIT 1)
WHERE e.email=$1
"#,
            email
        )
        .fetch_optional(conn)
        .await?;
        Ok(component)
    }

    async fn get_customer_with_email_by_username(
        &self,
        conn: &mut PgConnection,
        username: &str,
    ) -> Result<Option<(CustomerComponent, EmailComponent)>> {
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
        .fetch_optional(conn)
        .await?;
        row.map(|row| {
            let user = CustomerComponent {
                id: row.customer_id,
                username: row.username,
                preferences: serde_json::from_value(row.preferences)?,
                created_at: row.customer_created_at,
            };
            let email = EmailComponent {
                id: row.email_id,
                customer_id: row.email_customer_id,
                email: row.email,
                created_at: row.email_created_at,
            };
            Ok((user, email))
        })
        .transpose()
    }

    async fn create_email(&self, conn: &mut PgConnection, email: &str, customer_id: Uuid) -> Result<EmailComponent> {
        let component = EmailComponent {
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
            component.id,
            component.customer_id,
            component.email,
            component.created_at
        )
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
