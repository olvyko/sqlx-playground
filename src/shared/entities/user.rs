use super::*;

#[derive(Debug)]
pub struct UserEntity {
    pub id: Uuid,
    pub username: String,
    pub preferences: Json<PreferencesEntity>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PreferencesEntity {
    pub online: bool,
    pub sounds: bool,
}
