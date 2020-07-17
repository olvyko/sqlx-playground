use super::*;

#[derive(Debug)]
pub struct Customer {
    pub id: Uuid,
    pub username: String,
    pub preferences: Preferences,
    pub created_at: NaiveDateTime,

    pub email: Option<Email>,
}

#[derive(Debug)]
pub struct Preferences {
    pub online: bool,
    pub sounds: bool,
}

impl From<PreferencesEntity> for Preferences {
    fn from(preferences: PreferencesEntity) -> Preferences {
        Preferences {
            online: preferences.online,
            sounds: preferences.sounds,
        }
    }
}

impl From<(CustomerEntity, Option<EmailEntity>)> for Customer {
    fn from((customer, email): (CustomerEntity, Option<EmailEntity>)) -> Customer {
        Customer {
            id: customer.id,
            username: customer.username,
            preferences: customer.preferences.0.into(),
            created_at: customer.created_at,
            email: email.map(|e| e.into()),
        }
    }
}
