use async_trait::async_trait;
use shared::*;

#[async_trait]
pub trait EntryDb {
    async fn create_customer(&self, conn: &mut PgConnection, username: &str) -> Result<CustomerComponent>;
    async fn get_customer_by_username(&self, conn: &mut PgConnection, username: &str) -> Result<Option<CustomerComponent>>;
    async fn get_customer_by_email(&self, conn: &mut PgConnection, email: &str) -> Result<Option<CustomerComponent>>;
    async fn get_customer_with_email_by_username(
        &self,
        conn: &mut PgConnection,
        username: &str,
    ) -> Result<Option<(CustomerComponent, EmailComponent)>>;
    async fn create_email(&self, conn: &mut PgConnection, email: &str, customer_id: Uuid) -> Result<EmailComponent>;
}

pub trait EntryMailer {
    fn send_verification_mail(&self, to: &str, email_verify_token: &str) -> Result<()>;
}
