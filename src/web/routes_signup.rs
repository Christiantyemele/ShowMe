use crate::authentication::{login, signup, AuthState, SignupPayload};
use crate::error::{error_page, LoginError};
use crate::utils::login_response;
use crate::{delete_logged_in, Database, Random};
use axum::extract::Json;
use axum::{response::IntoResponse, Extension};
use http::StatusCode;
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
        Ok(session_tk) => Ok(login_response(cookie, session_tk).await),
        Err(err) => Err(error_page(&err)),
    }
}
pub async fn post_delete_me(
    Extension(mut database): Extension<Database>,
    Extension(authstate): Extension<AuthState>,
) -> impl IntoResponse {
    if authstate.logged_in() {
        let state = authstate.0.unwrap();
        delete_logged_in(&mut database, state.0).await.unwrap();
        StatusCode::OK
    } else {
        return Err(LoginError::NotLogging).unwrap();
    
    }
}
