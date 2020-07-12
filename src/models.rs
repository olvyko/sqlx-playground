use serde::{Deserialize, Serialize};
use sqlx::types::{chrono::NaiveDateTime, Json, Uuid};

pub type EntityId = Uuid;

#[derive(Debug)]
pub struct UserEntity {
    pub id: EntityId,
    pub username: String,
    pub preferences: Json<UserPreferences>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    pub profit: bool,
    pub wagered: bool,
    pub online: bool,
    pub sounds: bool,
}
