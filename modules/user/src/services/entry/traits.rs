use async_trait::async_trait;
use shared::*;

#[async_trait]
pub trait EntryDb {
    async fn create_customer(&self, conn: &mut PgConnection, username: &str) -> Result<CustomerComp>;
    async fn get_customer_by_username(&self, conn: &mut PgConnection, username: &str) -> Result<Option<CustomerComp>>;
    async fn get_customer_by_email(&self, conn: &mut PgConnection, email: &str) -> Result<Option<CustomerComp>>;
    async fn get_customer_with_email_by_username(
        &self,
        conn: &mut PgConnection,
        username: &str,
    ) -> Result<Option<(CustomerComp, Option<EmailComp>)>>;
    async fn create_email(&self, conn: &mut PgConnection, email: &str, customer_id: Uuid) -> Result<EmailComp>;
}

pub trait EntryMailer {
    fn send_verification_mail(&self, to: &str, email_verify_token: &str) -> Result<()>;
}
