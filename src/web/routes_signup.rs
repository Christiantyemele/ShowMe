use crate::authentication::{login, signup, SignupPayload};
use crate::error::error_page;
use crate::utils::login_response;
use crate::{Database, Random};
use axum::extract::Json;
use axum::{response::IntoResponse, Extension};
use tower_cookies::Cookies;

pub async fn post_signup(
    Extension(database): Extension<Database>,
    Json(signup_payload): Json<SignupPayload>,
) -> impl IntoResponse {
    match signup(database, signup_payload.username, signup_payload.password).await {
        Ok(_) => Ok(()),
        Err(e) => Err(error_page(&e)),
    }
}
pub async fn post_login(
    Extension(cookie): Extension<Cookies>,
    Extension(database): Extension<Database>,
    Extension(random): Extension<Random>,
    Json(login_payload): Json<SignupPayload>,
) -> impl IntoResponse {
    match login(
        database,
        login_payload.username,
        random,
        login_payload.password,
    )
    .await
    {
        Ok(session_tk) => Ok(login_response(cookie, session_tk)),
        Err(err) => Err(error_page(&err)),
    }
}
