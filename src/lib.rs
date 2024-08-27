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
pub fn update_user(conn: &mut PgConnection, usernam: String) {
    use schema::users::dsl::*;
    let update = diesel::update(users.filter(username.like(usernam)))
        .set(schema::users::dsl::username.eq("string1"))
        .returning(Users::as_returning())
        .get_result(conn);
}
pub fn delete_user(conn: &mut PgConnection, usernam: String) {
    use schema::users::dsl::*;
    let num_deleted = diesel::delete(users.filter(username.like(usernam)))
        .execute(conn)
        .expect("Error deleting posts");
    println!("Deleted {} posts", num_deleted)
}
