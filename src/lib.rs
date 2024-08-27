use std::env;

use diesel::{
    Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
    TextExpressionMethods,
};
use dotenvy::dotenv;
use model::{NewUser, Users};

pub mod model;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
pub fn create_user(conn: &mut PgConnection, user: String, pass: String) -> Users {
    let new_user = NewUser {
        username: user,
        passkey: pass,
    };
    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .returning(Users::as_returning())
        .get_result(conn)
        .expect("Error saving new user")
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
