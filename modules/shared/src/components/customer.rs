use super::*;

#[derive(Debug, sqlx::FromRow)]
pub struct CustomerComponent {
    pub id: Uuid,
    pub username: String,
    pub customer_type: CustomerTypeComponent,
    pub preferences: Json<PreferencesComponent>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[repr(i16)]
pub enum CustomerTypeComponent {
    Admin = 1,
    User = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferencesComponent {
    pub online: bool,
    pub sounds: bool,
}
