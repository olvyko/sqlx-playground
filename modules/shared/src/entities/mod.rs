mod customer;
mod email;

pub use customer::*;
pub use email::*;

pub(self) use crate::scalars::*;
pub(self) use serde::{Deserialize, Serialize};
