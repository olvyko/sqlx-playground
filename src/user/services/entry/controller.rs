use super::EntryUsecase;
use crate::shared::*;
use sqlx::PgConnection;

pub struct EntryController;

impl EntryController {
    pub fn new() -> EntryController {
        EntryController
    }

    pub async fn register(&self, conn: &mut PgConnection, username: &str, email: Option<&str>) -> Result<Customer> {
        if conn.get_customer_by_username(username).await?.is_some() {
            return Err("Username already occupied")?;
        };
        let customer = conn.create_customer(username).await?;
        let email = if let Some(email) = email {
            if conn.get_customer_by_email(email).await?.is_some() {
                return Err("Email already occupied")?;
            };
            Some(conn.create_email(email, customer.id).await?)
        } else {
            None
        };
        Ok((customer, email).into())
    }
}
