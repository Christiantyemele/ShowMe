use std::{error::Error, fmt::Display};

use axum::response::Response;
#[derive(Debug)]

pub enum SignupError {
    InvalidUsername,
    UserNameTaken,
    InternalError,
    InvalidPassword,
}
impl Display for SignupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignupError::InternalError => f.write_str("Internal Error"),
            SignupError::UserNameTaken => f.write_str("Username already exits"),
            SignupError::InvalidPassword => f.write_str("Invalid Password"),
            SignupError::InvalidUsername => f.write_str("Invalid Username"),
        }
    }
}
impl Error for SignupError {}
pub fn error_page(err: &dyn std::error::Error) -> impl axum::response::IntoResponse {
    Response::builder()
        .status(http::StatusCode::INTERNAL_SERVER_ERROR)
        .body(format!("Err: {}", err))
        .unwrap()
}
