use crate::services::context::JuniperContext;
use juniper::{graphql_object, FieldResult};
use shared::*;

pub struct UserQueries;

#[graphql_object(context = JuniperContext, scalar = ServerScalarValue)]
impl UserQueries {
    /// Return customer
    async fn get_customer_by_username(ctx: &JuniperContext, username: String) -> FieldResult<Option<Customer>, ServerScalarValue> {
        let mut conn = ctx.db_pool().acquire().await?;
        let customer = ctx.modules().user().entry().get_customer_by_username(&mut conn, &username).await?;
        Ok(customer)
    }
}
