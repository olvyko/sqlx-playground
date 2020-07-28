use async_trait::async_trait;
use shared::*;

#[async_trait]
pub trait EntryDbGateway {
    async fn create_customer(&self, conn: &mut PgConnection, username: &str) -> Result<Customer>;
    async fn get_customer_by_username(&self, conn: &mut PgConnection, username: &str) -> Result<Option<Customer>>;
    async fn get_customer_by_email(&self, conn: &mut PgConnection, email: &str) -> Result<Option<Customer>>;
    async fn get_customer_with_email_by_username(
        &self,
        conn: &mut PgConnection,
        username: &str,
    ) -> Result<Option<(Customer, Option<Email>)>>;
    async fn create_email(&self, conn: &mut PgConnection, email: &str, customer_id: Uuid) -> Result<Email>;
}

pub trait EntryMailerGateway {
    fn send_verification_mail(&self, to: &str, email_verify_token: &str) -> Result<()>;
}
