//! Utility functions

#[macro_use]
pub mod errors;
pub mod time;
#[macro_use]
pub mod crud;

pub use self::errors::{AppResult, ErrorTemplateContext,error_page};
pub use self::time::date_from_str;
