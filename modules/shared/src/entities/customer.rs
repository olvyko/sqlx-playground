use super::*;

#[derive(Debug)]
pub struct CustomerEntity {
    pub id: Uuid,
    pub username: String,
    pub preferences: Json<PreferencesEntity>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferencesEntity {
    pub online: bool,
    pub sounds: bool,
}
