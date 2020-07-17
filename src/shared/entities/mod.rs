mod customer;
mod email;

pub use customer::*;
pub use email::*;

pub(self) use crate::shared::scalars::*;
pub(self) use serde::{Deserialize, Serialize};
