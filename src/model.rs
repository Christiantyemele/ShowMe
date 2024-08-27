use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub passkey: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub passkey: String,
}
#[derive(Insertable)]
#[diesel(table_name = crate::schema::sessions)]

pub struct Session {
    pub user_id: i32,
    pub session_token: Vec<u8>,
}
