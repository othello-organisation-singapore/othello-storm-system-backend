#[derive(Debug, PartialEq)]
pub enum ErrorType {
    AuthenticationFailed,
    DatabaseError,
    TokenExpired,
    PermissionDenied,
    BadRequestError(String),
    ExternalConnectionError(String),
    UnknownError(String),
}

impl ErrorType {
    pub fn to_error_code(&self) -> i32 {
        match self {
            ErrorType::UnknownError(_) => 1,
            ErrorType::BadRequestError(_) => 2,
            ErrorType::AuthenticationFailed => 3,
            ErrorType::TokenExpired => 4,
            ErrorType::PermissionDenied => 5,
            ErrorType::DatabaseError => 6,
            ErrorType::ExternalConnectionError(_) => 7,
        }
    }

    pub fn to_error_message(&self) -> String {
        match self {
            ErrorType::UnknownError(message) => message.clone(),
            ErrorType::BadRequestError(message) => String::from(
                format!("Bad request: {}.", message)
            ),
            ErrorType::ExternalConnectionError(message) => String::from(
                format!("Cannot connect to external source ({}), please try again.", message)
            ),
            ErrorType::AuthenticationFailed => String::from("Authentication failed."),
            ErrorType::PermissionDenied => String::from(
                "You didn't have permission to do this action."
            ),
            ErrorType::DatabaseError => String::from(
                "There is an error in handling the database, please try again \
                or contact administrator if this error persists."
            ),
            ErrorType::TokenExpired => String::from("Login expired. Please login again."),
        }
    }
}
