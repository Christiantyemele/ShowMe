use std::{
    env,
    sync::{Arc, Mutex},
};

use diesel::{
    Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
    TextExpressionMethods,
};
use dotenvy::dotenv;
use model::{NewUser, Session, Users};
use rand_chacha::ChaCha8Rng;
use rand_core::RngCore;

pub mod authentication;
pub mod error;
pub mod model;
pub mod schema;
pub mod web;

type Random = Arc<Mutex<ChaCha8Rng>>;
type Database = PgConnection;
impl Clone for PgConnection {
    fn clone(&self) -> Self {
        Self {PgConnection}
    }
}

#[derive(Clone)]
pub struct SessionToken(u128);
impl SessionToken {
    pub fn generate_new(random: Random) -> Self {
        let mut u128_pool = [0u8; 16];
        random.lock().unwrap().fill_bytes(&mut u128_pool);
        Self(u128::from_le_bytes(u128_pool))
    }
    pub fn into_database_value(self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
pub fn create_user(
    conn: &mut PgConnection,
    user: String,
    pass: String,
) -> Result<i32, diesel::result::Error> {
    let new_user = NewUser {
        username: user,
        passkey: pass,
    };
    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .returning(schema::users::id)
        .get_result(conn)
}

pub fn delete_user(conn: &mut PgConnection, usernam: String) {
    use schema::users::dsl::*;
    let _ = diesel::delete(users.filter(username.like(usernam)))
        .execute(conn)
        .expect("Error deleting user");
}
pub fn get_user(conn: &mut PgConnection, usernam: String) -> Vec<Users> {
    use schema::users::dsl::*;
    let single_user = users
        .filter(username.eq(usernam))
        .select(Users::as_select())
        .load(conn)
        .expect("Error loading user");
    single_user
}
pub fn get_all_users(conn: &mut PgConnection) -> Vec<Users> {
    use schema::users::dsl::*;
    let all_users = users
        .select(Users::as_select())
        .load(conn)
        .expect("Error loading users");
    all_users
}
pub fn create_session(
    conn: &mut PgConnection,
    token: SessionToken,
    uid: i32,
) -> Vec<u8>{
    let new_session = Session {
        user_id: uid,
        session_token: token.into_database_value(),
    };
    diesel::insert_into(schema::sessions::table)
        .values(&new_session)
        .returning(schema::sessions::dsl::session_token)
        .get_result(conn).unwrap()
}
