use crate::{
    authentication::new_session, create_user, error::SignupError, get_user, Database, Random,
    SessionToken,
};
use pbkdf2::{
    password_hash::{PasswordHasher, SaltString},
    Pbkdf2,
};
use rand_core::OsRng;

pub struct LoginPayload {
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
        let result = create_user(&mut database, username, hashed_password);
        let new_user_id = match result {
            Ok(uid) => uid,

            _ => {
                return Err(SignupError::InternalError);
            }
        };
        Ok(new_session(&database, random, new_user_id).await)
    }
}
