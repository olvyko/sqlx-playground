use super::*;
use crate::components::*;
use crate::scalars::*;
use juniper::*;

#[derive(Debug, GraphQLObject)]
#[graphql(scalar = DefaultScalarValue)]
pub struct Customer {
    pub id: Uuid,
    pub username: String,
    pub preferences: Preferences,
    pub created_at: UnixTime,
    pub email: Option<Email>,
}

#[derive(Debug, GraphQLObject)]
pub struct Preferences {
    pub online: bool,
    pub sounds: bool,
}

impl From<PreferencesComponent> for Preferences {
    fn from(preferences: PreferencesComponent) -> Preferences {
        Preferences {
            online: preferences.online,
            sounds: preferences.sounds,
        }
    }
}

impl From<CustomerComponent> for Customer {
    fn from(customer: CustomerComponent) -> Customer {
        Customer {
            id: customer.id,
            username: customer.username,
            preferences: customer.preferences.0.into(),
            created_at: customer.created_at.into(),
            email: None,
        }
    }
}

impl From<(CustomerComponent, Option<EmailComponent>)> for Customer {
    fn from((customer, email): (CustomerComponent, Option<EmailComponent>)) -> Customer {
        Customer {
            id: customer.id,
            username: customer.username,
            preferences: customer.preferences.0.into(),
            created_at: customer.created_at.into(),
            email: email.map(|e| e.into()),
        }
    }
}
