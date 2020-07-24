use crate::scalars::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow)]
pub struct CustomerComp {
    pub id: Uuid,
    pub username: String,
    pub customer_type: CustomerTypeComp,
    pub preferences: Json<PreferencesComp>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[repr(i16)]
pub enum CustomerTypeComp {
    Admin = 1,
    User = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferencesComp {
    pub online: bool,
    pub sounds: bool,
}
