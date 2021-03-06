use crate::scalars::*;

#[derive(Clone, Debug)]
pub struct Email {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub email: String,
    pub created_at: NaiveDateTime,
}
