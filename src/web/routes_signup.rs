use axum::{response::IntoResponse, Extension, Json};
use tower_cookies::Cookies;

use crate::utils::login_response;
use crate::Database;
use crate::{
    authentication::{signup, SignupPayload},
    error::error_page,
    Random,
};

async fn post_signup(
    cookie: Cookies,
    Extension(database): Extension<Database>,
    Extension(random): Extension<Random>,
    Json(signup_payload): Json<SignupPayload>,
) -> impl IntoResponse {
    match signup(
        database,
        random,
        signup_payload.username,
        signup_payload.password,
    )
    .await
    {
        Ok(session_tk) => Ok(login_response(cookie, session_tk)),
        Err(e) => Err(error_page(&e)),
    }
}
