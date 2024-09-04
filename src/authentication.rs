use axum::extract::Request;

use pbkdf2::{
    password_hash::{PasswordHash, PasswordVerifier, SaltString},
    Pbkdf2,
};
use rand_core::OsRng;

use crate::{
    create_session, create_user,
    error::{LoginError, SignupError},
    get_id_pwd, get_user, Database, Random, SessionToken, AUTH_COOKIE_NAME,
};
use pbkdf2::password_hash::PasswordHasher;

#[derive(Clone)]
pub struct User {
    pub username: String,
}
#[allow(unused)]
#[derive(Clone)]
pub struct AuthState(Option<(SessionToken, Option<User>, Database)>);

pub async fn new_session(mut database: Database, random: Random, uid: i32) -> SessionToken {
    let session_token = SessionToken::generate_new(random);
    create_session(&mut database, session_token.clone(), uid).await;
    session_token
}
// **AUTH MIDDLEWARE**
pub async fn auth(
    database: Database,
    mut req: Request,
    next: axum::middleware::Next,
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
        .insert(AuthState(session_tk.map(|v| (v, None, database))));
    next.run(req).await
}
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct SignupPayload {
    pub username: String,
    pub password: String,
}
pub async fn signup(
    mut database: Database,
    username: String,
    password: String,
) -> Result<(), SignupError> {
    fn valid_username(username: &str) -> bool {
        (1..20).contains(&username.len())
            && username
                .chars()
                .all(|c| matches!(c, 'a' ..='z' | '0'..='9' | '-'))
    }

    if !valid_username(&username) {
        return Err(SignupError::InvalidUsername);
    }
    if valid_username(&username) && get_user(&mut database, username.clone()).await.len() != 0 {
        return Err(SignupError::UserNameTaken);
    } else {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Pbkdf2.hash_password(password.as_bytes(), &salt);
        let hashed_password = if let Ok(password) = password_hash {
            password.to_string()
        } else {
            return Err(SignupError::InvalidPassword);
        };
        let result = create_user(&mut database, username.clone(), hashed_password).await;
        let _new_user_id = match result {
            Ok(uid) => uid,

            _ => {
                return Err(SignupError::InternalError);
            }
        };
        Ok(())
    }
}
pub async fn login(
    mut database: Database,
    username: String,
    random: Random,
    password: String,
) -> Result<SessionToken, LoginError> {
    let row = get_id_pwd(&mut database, username).await;
    let (user_id, hashed_password) = if let Some(row) = row {
        row
    } else {
        return Err(LoginError::UserDoesNotExists);
    };
    let verified_hash = PasswordHash::new(&hashed_password).unwrap();
    if let Err(_err) = Pbkdf2.verify_password(password.as_bytes(), &verified_hash) {
        return Err(LoginError::WrongPassword);
    }
    Ok(new_session(database, random, user_id).await)
}
pub async fn delete_me(auth_state: AuthState) {
    
}