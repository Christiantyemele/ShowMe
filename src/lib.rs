use diesel::prelude::*;
use diesel_async::{pooled_connection::deadpool::Pool, RunQueryDsl};
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use dotenvy::dotenv;
use model::{NewUser, Session, Users};
use rand_chacha::ChaCha8Rng;
use std::{
    env,
    str::FromStr,
    sync::{Arc, Mutex},
};

use rand_core::RngCore;

pub mod authentication;
pub mod error;
pub mod model;
pub mod schema;
pub mod utils;
pub mod web;

type Random = Arc<Mutex<ChaCha8Rng>>;
const AUTH_COOKIE_NAME: &str = "auth_token";
type Database = Pool<AsyncPgConnection>;
pub type SharedDb = Pool<AsyncPgConnection>;

pub struct AppState {
    pub database: SharedDb,
}
#[derive(Clone)]
pub struct SessionToken(u128);
impl FromStr for SessionToken {
    type Err = <u128 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}
impl SessionToken {
    pub fn into_cookie_value(self) -> String {
        self.0.to_string()
    }
    pub fn generate_new(random: Random) -> Self {
        let mut u128_pool = [0u8; 16];
        random.lock().unwrap().fill_bytes(&mut u128_pool);
        Self(u128::from_le_bytes(u128_pool))
    }
    pub fn into_database_value(self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

pub async fn establish_connection() -> Pool<AsyncPgConnection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    Pool::builder(manager)
        .build()
        .expect("Could not build connection pool")
}

pub async fn create_user(
    conn: &mut Database,
    user: String,
    pass: String,
) -> Result<i32, diesel::result::Error> {
    use diesel_async::RunQueryDsl;
    let mut conn = conn.get().await.unwrap();

    let new_user = NewUser {
        username: user,
        passkey: pass,
    };
    let result: Result<i32, diesel::result::Error> = diesel::insert_into(schema::users::table)
        .values(&new_user)
        .returning(schema::users::id)
        .get_result::<i32>(&mut *conn)
        .await;
    result
}

pub async fn delete_user(conn: &mut Database, usernam: String) {
    let mut conn = conn.get().await.unwrap();
    use schema::users::dsl::*;
    diesel::delete(users.filter(username.like(usernam)))
        .execute(&mut conn)
        .await
        .expect("Error deleting user");
}
pub async fn get_user(conn: &mut Database, usernam: String) -> Vec<Users> {
    use schema::users::dsl::*;
    let conn = conn.get().await.unwrap();
    let mut conn = conn;
    let result = users
        .filter(username.eq(usernam))
        .select(Users::as_select())
        .load(&mut *conn)
        .await
        .unwrap();

    result
}
pub async fn get_all_users(conn: &mut Database) -> Vec<Users> {
    let mut conn = conn.get().await.unwrap();
    use schema::users::dsl::*;
    let all_users = users
        .select(Users::as_select())
        .get_results(&mut *conn)
        .await
        .unwrap();
    all_users
}
pub async fn create_session(conn: &mut Database, token: SessionToken, uid: i32) -> Vec<u8> {
    let mut conn = conn.get().await.unwrap();
    let new_session = Session {
        user_id: uid,
        session_token: token.into_database_value(),
    };
    let result = diesel::insert_into(schema::sessions::table)
        .values(&new_session)
        .returning(schema::sessions::dsl::session_token)
        .get_result::<Vec<u8>>(&mut *conn)
        .await
        .unwrap();
    result
}
