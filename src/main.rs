use std::sync::{Arc, Mutex};

use diesel::prelude::*;
use gp::*;
use model::*;
use rand_chacha::ChaCha8Rng;
use schema::users;

type Random = Arc<Mutex<ChaCha8Rng>>;
fn main() {
    use self::schema::users::dsl::*;
    let connection = &mut establish_connection();
}
