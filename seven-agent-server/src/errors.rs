use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::Request;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("User not found")]
    UserNotFound,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Password hashing error")]
    PasswordError,

    #[error("Internal server error")]
    InternalServerError,
}

impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let (status, message) = match self {
            AppError::InvalidCredentials => (Status::Unauthorized, "Invalid credentials"),
            AppError::UserAlreadyExists => (Status::BadRequest, "User already exists"),
            AppError::InvalidToken => (Status::Unauthorized, "Invalid token"),
            AppError::UserNotFound => (Status::NotFound, "User not found"),
            AppError::DatabaseError(_) => (Status::InternalServerError, "Database error"),
            AppError::PasswordError => (Status::InternalServerError, "Password processing error"),
            AppError::InternalServerError => (Status::InternalServerError, "Internal server error"),
        };

        Response::build()
            .status(status)
            .sized_body(message.len(), std::io::Cursor::new(message))
            .ok()
    }
}