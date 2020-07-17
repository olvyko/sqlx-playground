mod customer;
mod email;

pub use customer::*;
pub use email::*;

pub(super) use crate::shared::scalars::*;
pub(super) use serde::{Deserialize, Serialize};
pub use sqlx::types::Json;
