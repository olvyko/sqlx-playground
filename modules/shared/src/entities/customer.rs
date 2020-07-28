use super::*;
use crate::components::*;
use crate::scalars::*;
use juniper::*;

#[derive(Debug, GraphQLObject)]
#[graphql(scalar = ServerScalarValue)]
pub struct CustomerEntity {
    pub id: Uuid,
    pub username: String,
    pub customer_type: CustomerTypeEntity,
    pub preferences: PreferencesEntity,
    pub created_at: NaiveDateTime,
    pub email: Option<EmailEntity>,
}

#[derive(Debug, Clone, Copy, GraphQLEnum)]
pub enum CustomerTypeEntity {
    Admin = 1,
    Player = 2,
}

#[derive(Debug, GraphQLObject)]
pub struct PreferencesEntity {
    pub online: bool,
    pub sounds: bool,
}

impl From<CustomerType> for CustomerTypeEntity {
    fn from(customer_type: CustomerType) -> CustomerTypeEntity {
        match customer_type {
            CustomerType::Admin => CustomerTypeEntity::Admin,
            CustomerType::Player => CustomerTypeEntity::Player,
        }
    }
}

impl From<Preferences> for PreferencesEntity {
    fn from(preferences: Preferences) -> PreferencesEntity {
        PreferencesEntity {
            online: preferences.online,
            sounds: preferences.sounds,
        }
    }
}

impl From<(Customer, Option<Email>)> for CustomerEntity {
    fn from((customer, email): (Customer, Option<Email>)) -> CustomerEntity {
        CustomerEntity {
            id: customer.id,
            username: customer.username,
            customer_type: customer.customer_type.into(),
            preferences: customer.preferences.0.into(),
            created_at: customer.created_at,
            email: email.map(|e| e.into()),
        }
    }
}
