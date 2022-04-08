extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;

use random_string::generate;

fn main() {
  let connection = establish_connection();

  let charset: String = String::from("1234567890qwertyuiasdfghjklzxcvbnmQWERTYUIASDFGHJKLZXCVBNM");
  let phone_charset: String = String::from("1234567890");

  let name: String = String::from( generate(12, charset));
  let phone: String = String::from(generate(10, phone_charset));

  create_user(&connection, &name, &phone);
}
