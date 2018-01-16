use diesel::result::Error as diesel_error;
use chrono::ParseError as chrono_error;
use failure::Fail;

#[derive(Fail,Debug)]
pub enum AppError {
    #[fail(display = "Database error occurred: {}", error)]
    DatabaseError {
        #[cause] error: diesel_error,
    },
    #[fail(display = "Serialization error occurred")]
    SerializationError,
    #[fail(display = "Parsing (time) error occurred: {}", error)]
    TimeParseError {
        #[cause] error: chrono_error
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize, Debug)]
pub struct ErrorTemplateContext {
    message: String,
    cause: String,
    backtrace: String
}

impl ErrorTemplateContext {
    pub fn from(error: AppError) -> ErrorTemplateContext {
        ErrorTemplateContext {
            message: format!("{}", error),
            cause: format!("{:?}", error.cause()),
            backtrace: format!("{:?}", error.backtrace())
        }
    }
}