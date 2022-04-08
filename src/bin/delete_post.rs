extern crate diesel_demo;
extern crate diesel;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use std::env::args;


fn main() {
  use diesel_demo::schema::posts::dsl::{posts, id};
  
  let id_params = args().nth(1).expect("deleting post  requires a post id")
  .parse::<i32>().expect("Invalid ID");

  let connection = establish_connection();

  diesel::delete(posts.filter(id.eq(id_params))).execute(&connection).expect("Error deleting user");
}
