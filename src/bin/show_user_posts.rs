extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use diesel::prelude::*;

use self::models::{User, Post};
fn main() {
    use diesel_demo::schema::users::dsl::*;

    let connection = establish_connection();

    let user = users.find(3).first::<User>(&connection).expect("Error loading user");
    let post_list = Post::belonging_to(&user)
        .load::<Post>(&connection)
        .expect("Error loading posts");
    
    println!("{:?}", post_list);
}