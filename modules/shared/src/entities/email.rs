use crate::components::*;
use crate::scalars::*;
use juniper::*;

#[derive(Clone, Debug, GraphQLObject)]
#[graphql(scalar = ServerScalarValue)]
pub struct Email {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub email: String,
    pub created_at: NaiveDateTime,
}

impl From<EmailComponent> for Email {
    fn from(email: EmailComponent) -> Email {
        Email {
            id: email.id,
            customer_id: email.customer_id,
            email: email.email,
            created_at: email.created_at,
        }
    }
}
