use super::*;

#[derive(Clone, Debug)]
pub struct Email {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub email: String,
    pub created_at: NaiveDateTime,
}

impl From<EmailEntity> for Email {
    fn from(email: EmailEntity) -> Email {
        Email {
            id: email.id,
            customer_id: email.customer_id,
            email: email.email,
            created_at: email.created_at,
        }
    }
}
