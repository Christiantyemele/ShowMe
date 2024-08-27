use diesel::prelude::*;
use gp::*;
use model::*;
use schema::users;

fn main() {
    use self::schema::users::dsl::*;
    let connection = &mut establish_connection();
    
    let results = users
        .select(Users::as_select())
        .load(connection)
        .expect("Error loading posts");

    for user in results {
        println!("{:?}", user)
    }
}
