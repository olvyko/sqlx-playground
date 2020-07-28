use crate::scalars::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow)]
pub struct Customer {
    pub id: Uuid,
    pub username: String,
    pub customer_type: CustomerType,
    pub preferences: Json<Preferences>,
    pub data: Json<UserData>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[repr(i16)]
pub enum CustomerType {
    Admin = 1,
    Player = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preferences {
    pub online: bool,
    pub sounds: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserData {
    Admin(AdminData),
    Player(PlayerData),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AdminData {
    A1 = 1,
    A2 = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerData {
    B1 = 1,
}
