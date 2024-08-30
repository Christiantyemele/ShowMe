use axum::{response::IntoResponse, Extension};
use axum::extract::Json;
use tower_cookies::Cookies;
use crate::error::error_page;
use crate::utils::login_response;
use crate::Database;
use crate::{
    authentication::{signup, SignupPayload},
  
    Random,
};

pub async fn post_signup(
    Extension(cookie): Extension<Cookies>,
    Extension(database): Extension<Database>,
    Extension(random): Extension<Random>,
    Json(signup_payload): Json<SignupPayload>,
) -> impl IntoResponse {
    match signup(
        database,
        random,
        signup_payload.username,
        signup_payload.password,
    ) .await
    {
        Ok(session_tk) => Ok(login_response(cookie, session_tk)),
        Err(e) => Err(error_page(&e)),
    }
    
}
