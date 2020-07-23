use crate::services::context::JuniperContext;
use juniper::{graphql_object, FieldResult};
use shared::*;

pub struct UserMutations;

#[graphql_object(context = JuniperContext)]
impl UserMutations {
    /// Register customer
    async fn register(ctx: &JuniperContext, username: String, email: Option<String>) -> FieldResult<Customer> {
        let mut conn = ctx.db_pool().acquire().await?;
        let customer = ctx
            .modules()
            .user()
            .entry()
            .register(&mut conn, &username, email.as_deref())
            .await?;
        Ok(customer)
    }
}
