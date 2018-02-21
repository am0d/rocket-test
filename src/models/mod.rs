//! Models and database interactions

pub mod category;
pub mod period;
pub mod transaction;
pub mod crud;
pub mod validate;

pub mod prelude {
    pub use super::crud::Crud;
    pub use super::validate::{Validate, ValidateResult};
}
