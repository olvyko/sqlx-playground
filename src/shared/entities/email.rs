use super::*;

#[derive(Clone, Debug)]
pub struct EmailEntity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub created_at: NaiveDateTime,
}
