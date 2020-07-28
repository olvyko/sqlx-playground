use super::gateways::*;
use async_trait::async_trait;
use shared::*;
use sqlx::Row;

pub struct EntryController;

#[async_trait]
impl EntryDbGateway for EntryController {
    async fn create_customer(&self, conn: &mut PgConnection, username: &str) -> Result<Customer> {
        let component = Customer {
            id: new_uuid(),
            username: username.to_owned(),
            customer_type: CustomerType::Player,
            preferences: Json(Preferences {
                online: true,
                sounds: true,
            }),
            data: Json(UserData::Player(PlayerData::B1)),
            created_at: now_unix_time(),
        };
        sqlx::query(
            r#"
            INSERT INTO customer(id, username, customer_type, preferences, data, created_at)
            VALUES($1, $2, $3, $4, $5)
        "#,
        )
        .bind(&component.id)
        .bind(&component.username)
        .bind(&component.customer_type)
        .bind(&component.preferences)
        .bind(&component.data)
        .bind(&component.created_at)
        .execute(conn)
        .await?;
        Ok(component)
    }

    async fn get_customer_by_username(&self, conn: &mut PgConnection, username: &str) -> Result<Option<Customer>> {
        let component = sqlx::query_as(
            r#"
            SELECT
                id, username, customer_type, preferences, data, created_at
            FROM customer
            WHERE username=$1
        "#,
        )
        .bind(username)
        .fetch_optional(conn)
        .await?;
        Ok(component)
    }

    async fn get_customer_by_email(&self, conn: &mut PgConnection, email: &str) -> Result<Option<Customer>> {
        let component = sqlx::query_as(
            r#"
            SELECT
                c.id, c.username, c.customer_type, c.preferences, c.data, c.created_at
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
    ) -> Result<Option<(Customer, Option<Email>)>> {
        let components = sqlx::query(
            r#"
            SELECT
                c.id, c.username, c.customer_type, c.preferences, c.data, c.created_at,
                e.id, e.customer_id, e.email, e.created_at
            FROM customer AS c
                LEFT JOIN email AS e ON e.id=(SELECT id FROM email WHERE customer_id=c.id ORDER BY created_at DESC LIMIT 1)
            WHERE c.username=$1
        "#,
        )
        .bind(username)
        .try_map(|row: sqlx::postgres::PgRow| {
            let user = Customer {
                id: row.try_get(0)?,
                username: row.try_get(1)?,
                customer_type: row.try_get(2)?,
                preferences: row.try_get(3)?,
                data: row.try_get(4)?,
                created_at: row.try_get(5)?,
            };
            let email = match row.try_get::<Option<Uuid>, _>(6)?.is_some() {
                true => Some(Email {
                    id: row.try_get(7)?,
                    customer_id: row.try_get(8)?,
                    email: row.try_get(9)?,
                    created_at: row.try_get(10)?,
                }),
                false => None,
            };
            Ok((user, email))
        })
        .fetch_optional(conn)
        .await?;
        Ok(components)
    }

    async fn create_email(&self, conn: &mut PgConnection, email: &str, customer_id: Uuid) -> Result<Email> {
        let component = Email {
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
        .bind(&component.id)
        .bind(&component.customer_id)
        .bind(&component.email)
        .bind(&component.created_at)
        .execute(conn)
        .await?;
        Ok(component)
    }
}

#[allow(unused)]
impl EntryMailerGateway for EntryController {
    fn send_verification_mail(&self, to: &str, email_verify_token: &str) -> Result<()> {
        Ok(())
    }
}
