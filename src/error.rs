use std::{error::Error, fmt::Display};

use axum::response::{IntoResponse, Response};
use cookie::time::error;
#[derive(Debug)]

pub enum SignupError {
    InvalidUsername,
    UserNameTaken,
    InternalError,
    InvalidPassword,
}
#[derive(Debug)]
pub enum LoginError {
    UserDoesNotExists,
    WrongPassword,
    MissingDetails,

    NotLogging,
}
impl Display for SignupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignupError::InternalError => {
                print!("");
                f.write_str("Internal Error")
            }
            SignupError::UserNameTaken => f.write_str("Username already exits"),
            SignupError::InvalidPassword => f.write_str("Invalid Password"),
            SignupError::InvalidUsername => {
                println!("invalid");
                f.write_str("Invalid Username")
            }
        }
    }
}
impl Error for SignupError {}
pub fn error_page(err: &dyn std::error::Error) -> impl IntoResponse {
    Response::builder()
        .status(http::StatusCode::INTERNAL_SERVER_ERROR)
        .body(format!("Err: {}", err))
        .unwrap()
}
impl Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginError::UserDoesNotExists => f.write_str("User does not exist"),
            LoginError::MissingDetails => f.write_str("Missing details"),
            LoginError::WrongPassword => f.write_str("Wrong password"),
            LoginError::NotLogging => f.write_str("You are Not Logged In")
        }
    }
}

impl Error for LoginError {}
