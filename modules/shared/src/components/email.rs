use super::*;

#[derive(Clone, Debug)]
pub struct EmailComponent {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub email: String,
    pub created_at: NaiveDateTime,
}
