use super::*;
use shared::*;
use sqlx::PgConnection;

pub struct EntryUsecase {
    ctr: EntryController,
}

impl EntryUsecase {
    pub fn new() -> EntryUsecase {
        EntryUsecase { ctr: EntryController }
    }

    pub async fn register(&self, conn: &mut PgConnection, username: &str, email: Option<&str>) -> Result<CustomerEntity> {
        if self.ctr.get_customer_by_username(conn, username).await?.is_some() {
            return Err("Username already occupied")?;
        };
        let customer = self.ctr.create_customer(conn, username).await?;
        let email = match email {
            Some(email) => {
                if self.ctr.get_customer_by_email(conn, email).await?.is_some() {
                    return Err("Email already occupied")?;
                };
                let email = self.ctr.create_email(conn, email, customer.id).await?;
                self.ctr.send_verification_mail(&email.email, "token")?;
                Some(email)
            }
            None => None,
        };
        Ok((customer, email).into())
    }

    pub async fn get_customer_by_username(&self, conn: &mut PgConnection, username: &str) -> Result<Option<CustomerEntity>> {
        let customer = self.ctr.get_customer_by_username(conn, username).await?;
        Ok(customer.map(|c| (c, None).into()))
    }

    pub async fn get_customer_with_email_by_username(&self, conn: &mut PgConnection, username: &str) -> Result<Option<CustomerEntity>> {
        let customer = self.ctr.get_customer_with_email_by_username(conn, username).await?;
        Ok(customer.map(|c| c.into()))
    }
}
