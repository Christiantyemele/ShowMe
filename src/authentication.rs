
use axum::extract::Request;
use pbkdf2::{password_hash::SaltString, Pbkdf2};
use rand_core::OsRng;

use crate::{
    create_session, create_user, error::SignupError, get_user, Database, Random, SessionToken, AUTH_COOKIE_NAME
};
use pbkdf2::password_hash::PasswordHasher;

#[derive(Clone)]
pub struct User {
    pub username: String,
}
#[allow(unused)]
#[derive(Clone)]
pub struct AuthState(Option<(SessionToken, Option<User>)>);

pub async fn new_session(mut database: Database, random: Random, uid: i32) -> SessionToken {
    let session_token = SessionToken::generate_new(random);
    create_session(&mut database, session_token.clone(), uid);
    session_token
}
// **AUTH MIDDLEWARE**
pub async fn auth<B>(
    mut req: Request,
    next: axum::middleware::Next,
   // database: SharedDb,
) -> axum::response::Response {
    let session_tk = req
        .headers()
        .get_all("Cookie")
        .iter()
        .filter_map(|cookie| {
            cookie
                .to_str()
                .ok()
                .and_then(|cookie| cookie.parse::<cookie::Cookie>().ok())
        })
        .find_map(|cookie| {
            (cookie.name() == AUTH_COOKIE_NAME).then(move || cookie.value().to_owned())
        })
        .and_then(|cookie_value| cookie_value.parse::<SessionToken>().ok());
    req.extensions_mut()
        .insert(AuthState(session_tk.map(|v| (v, None))));
    next.run(req).await
}
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct SignupPayload {
    pub username: String,
    pub password: String,
}
pub async fn signup(
     mut database: Database,
    random: Random,
    username: String,
    password: String,
) -> Result<SessionToken, SignupError> {
    fn valid_username(username: &str) -> bool {
        (1..20).contains(&username.len())
            && username
                .chars()
                .all(|c| matches!(c, 'a' ..='z' | '0'..='9' | '-'))
    }

    if !valid_username(&username) {
        return Err(SignupError::InvalidUsername);
    }
    if valid_username(&username) && get_user(&mut database, username.clone()).len() != 0 {
        return Err(SignupError::UserNameTaken);
    } else {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Pbkdf2.hash_password(password.as_bytes(), &salt);
        let hashed_password = if let Ok(password) = password_hash {
            password.to_string()
        } else {
            return Err(SignupError::InvalidPassword);
        };
        let result = create_user(&mut database, username.clone(), hashed_password);
        let new_user_id = match result {
            Ok(uid) => uid,

            _ => {
                return Err(SignupError::InternalError);
            }
        };
        Ok(new_session(database, random, new_user_id).await)
    }
}
