#[derive(Debug, PartialEq)]
pub enum ErrorType {
    AuthenticationFailed,
    DatabaseConnectionError,
    PermissionDenied,
    BadRequestError(String),
    UnknownError(String),
}

impl ErrorType {
    pub fn to_error_code(&self) -> i32 {
        match self {
            ErrorType::UnknownError(_) => 2,
            ErrorType::BadRequestError(_) => 3,
            ErrorType::AuthenticationFailed => 4,
            ErrorType::PermissionDenied => 5,
            ErrorType::DatabaseConnectionError => 6,
            _ => 1,
        }
    }

    pub fn to_error_message(&self) -> String {
        match self {
            ErrorType::UnknownError(message) => message.clone(),
            ErrorType::BadRequestError(message) => String::from("Bad request: {}.", message),
            ErrorType::AuthenticationFailed => String::from("Authentication failed."),
            ErrorType::PermissionDenied => String::from("You didn't have permission to do this action."),
            ErrorType::DatabaseConnectionError => String::from("Cannot connect to database."),
            _ => String::from("Unidentified Error Type"),
        }
    }
}
