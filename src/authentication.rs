use crate::{create_session, Database, Random, SessionToken};

pub async fn new_session(mut database: Database, random: Random, uid: i32) -> SessionToken {
    let session_token = SessionToken::generate_new(random);
    create_session(&mut database, session_token.clone(), uid);
    session_token
}
