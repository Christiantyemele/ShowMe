use std::sync::Arc;

use crate::{
    authentication::new_session, create_user, error::SignupError, get_user, Database, Random,
    SessionToken,
};
use axum::{routing::post, Extension, Json, Router};
use pbkdf2::{
    password_hash::{PasswordHasher, SaltString},
    Pbkdf2,
};
use rand_core::OsRng;

#[derive(Clone)]
pub struct SignupPayload {
    pub username: String,
    pub password: String,
}
pub async fn signup(
    mut database: Database,
    random: Random,
    payload: Json<SignupPayload>,
) -> Result<SessionToken, SignupError> {
    fn valid_username(username: &str) -> bool {
        (1..20).contains(&username.len())
            && username
                .chars()
                .all(|c| matches!(c, 'a' ..='z' | '0'..='9' | '-'))
    }

    if !valid_username(&payload.username) {
        return Err(SignupError::InvalidUsername);
    }
    if valid_username(&payload.username)
        && get_user(&mut database, payload.username.clone()).len() != 0
    {
        return Err(SignupError::UserNameTaken);
    } else {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Pbkdf2.hash_password(payload.password.as_bytes(), &salt);
        let hashed_password = if let Ok(password) = password_hash {
            password.to_string()
        } else {
            return Err(SignupError::InvalidPassword);
        };
        let result = create_user(&mut database, payload.username.clone(), hashed_password);
        let new_user_id = match result {
            Ok(uid) => uid,

            _ => {
                return Err(SignupError::InternalError);
            }
        };
        Ok(new_session(database, random, new_user_id).await)
    }
}
pub fn routes(database: Database) -> Router{
    Router::new()
    .route("/api/signup", post(signup))
    .layer(Extension(database))

}