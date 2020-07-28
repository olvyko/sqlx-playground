use crate::components::*;
use crate::scalars::*;
use juniper::*;

#[derive(Clone, Debug, GraphQLObject)]
#[graphql(scalar = ServerScalarValue)]
pub struct EmailEntity {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub email: String,
    pub created_at: NaiveDateTime,
}

impl From<Email> for EmailEntity {
    fn from(email: Email) -> EmailEntity {
        EmailEntity {
            id: email.id,
            customer_id: email.customer_id,
            email: email.email,
            created_at: email.created_at,
        }
    }
}
