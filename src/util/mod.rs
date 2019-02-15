//! Utility functions

#[macro_use]
pub mod errors;
pub mod time;
#[macro_use]
pub mod crud;
pub mod response;

pub use self::errors::{error_page, AppResult, ErrorTemplateContext};
pub use self::time::date_from_str;

pub type Cents = i32;