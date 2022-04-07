extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;

// use std::io::{stdin, Read};
use random_string::generate;

fn main() {
  let connection = establish_connection();

  let title = String::from(get_random_string(12));
  let body = String::from(get_random_string(120));

  create_post(&connection, &title, &body);
}

fn get_random_string(length: usize) -> String {
  let charset = "1234567890qwertyuiasdfghjklzxcvbnmQWERTYUIASDFGHJKLZXCVBNM";

  generate(length, charset)
}