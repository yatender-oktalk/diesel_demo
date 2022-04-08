extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
// use self::models::*;
use diesel::prelude::*;

fn main() {
  use diesel_demo::schema::posts::dsl::*;

  let connection = establish_connection();
  let results = posts
      .select((title, body))
      .filter(published.eq(false))
      .filter(id.eq_any(vec![1,2,3,4]))
      .limit(5)
      .load::<(String, String)>(&connection);

  println!("Displaying {:?} posts", results);
}
