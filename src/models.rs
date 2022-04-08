use super::schema::{posts, users};

#[derive(Debug, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: Option<bool>,
    pub user_id: Option<i32>,
}

#[derive(Insertable, Debug)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub published: Option<&'a bool>,
    pub user_id: Option<&'a i32>,
}
#[derive(Debug, Queryable)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub phone: String,
}

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
  pub name: &'a str,
  pub phone: &'a str,
}
