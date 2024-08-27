use std::{
    env,
    sync::{Arc, Mutex},
};

use diesel::{
    Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
    TextExpressionMethods,
};
use dotenvy::dotenv;
use model::{NewUser, Users};
use rand_chacha::ChaCha8Rng;

pub mod error;
pub mod model;
pub mod schema;
pub mod web;
pub mod authentication;

type Random = Arc<Mutex<ChaCha8Rng>>;
type Database = PgConnection;
pub struct SessionToken(u128);

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
