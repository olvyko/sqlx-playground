use super::*;

#[derive(Debug)]
pub struct CustomerComponent {
    pub id: Uuid,
    pub username: String,
    pub preferences: Json<PreferencesComponent>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferencesComponent {
    pub online: bool,
    pub sounds: bool,
}
