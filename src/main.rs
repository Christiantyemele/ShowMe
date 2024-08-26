use diesel::prelude::*;
use gp::*;
use model::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .filter(username.eq("true"))
        .limit(5)
        .select(Users::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
}
