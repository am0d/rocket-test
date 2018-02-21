use diesel::result::Error as diesel_error;
use chrono::ParseError as chrono_error;
use failure::{Backtrace, Fail};
use rocket_contrib::Template;

#[derive(Fail, Debug)]
pub enum AppError {
    #[fail(display = "Database error occurred: {}", error)]
    DatabaseError {
        #[cause]
        error: diesel_error,
        backtrace: Backtrace,
    },
    #[fail(display = "Parsing (time) error occurred: {}", error)]
    TimeParseError {
        #[cause]
        error: chrono_error,
        backtrace: Backtrace,
    },
}

/// Create an error object, with an included backtrace
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

pub fn error_page(e: AppError) -> Template {
    let context = ErrorTemplateContext::from(e);
    Template::render("error", &context)
}
