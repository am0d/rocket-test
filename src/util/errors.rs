use diesel::result::Error as diesel_error;
use chrono::ParseError as chrono_error;
use failure::{Backtrace, Fail};

#[derive(Fail, Debug)]
pub enum AppError {
    #[fail(display = "Database error occurred: {}", error)]
    DatabaseError {
        #[cause] error: diesel_error,
        backtrace: Backtrace,
    },
    #[fail(display = "Parsing (time) error occurred: {}", error)]
    TimeParseError {
        #[cause] error: chrono_error,
        backtrace: Backtrace,
    },
}

#[macro_export]
macro_rules! app_error {
    ($error_type: ident, $error: expr) => ({
        use failure::Backtrace;
        use util;
        util::errors::AppError::$error_type {error: $error, backtrace: Backtrace::new()}
        })
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize, Debug)]
pub struct ErrorTemplateContext {
    message: String,
    cause: String,
    backtrace: String,
}

impl ErrorTemplateContext {
    pub fn from(error: AppError) -> ErrorTemplateContext {
        ErrorTemplateContext {
            message: format!("{}", error),
            cause: format!("{:?}", error.cause()),
            backtrace: format!("{:?}", error.backtrace()).replace("/n", "<br />"),
        }
    }
}
