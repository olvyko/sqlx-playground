use super::*;
use crate::components::*;
use crate::scalars::*;
use juniper::*;

#[derive(Debug, GraphQLObject)]
#[graphql(scalar = ServerScalarValue)]
pub struct Customer {
    pub id: Uuid,
    pub username: String,
    pub customer_type: CustomerType,
    pub preferences: Preferences,
    pub created_at: NaiveDateTime,
    pub email: Option<Email>,
}

#[derive(Debug, Clone, Copy, GraphQLEnum)]
pub enum CustomerType {
    Admin = 1,
    User = 2,
}

#[derive(Debug, GraphQLObject)]
pub struct Preferences {
    pub online: bool,
    pub sounds: bool,
}

impl From<CustomerTypeComponent> for CustomerType {
    fn from(customer_type: CustomerTypeComponent) -> CustomerType {
        match customer_type {
            CustomerTypeComponent::Admin => CustomerType::Admin,
            CustomerTypeComponent::User => CustomerType::User,
        }
    }
}

impl From<PreferencesComponent> for Preferences {
    fn from(preferences: PreferencesComponent) -> Preferences {
        Preferences {
            online: preferences.online,
            sounds: preferences.sounds,
        }
    }
}

impl From<(CustomerComponent, Option<EmailComponent>)> for Customer {
    fn from((customer, email): (CustomerComponent, Option<EmailComponent>)) -> Customer {
        Customer {
            id: customer.id,
            username: customer.username,
            customer_type: customer.customer_type.into(),
            preferences: customer.preferences.0.into(),
            created_at: customer.created_at,
            email: email.map(|e| e.into()),
        }
    }
}
