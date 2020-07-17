mod customer;
mod email;

pub use customer::*;
pub use email::*;

pub(self) use crate::shared::entities::*;
pub(self) use crate::shared::scalars::*;
