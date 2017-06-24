use db;
use schema::post;
use rocket::request::{self, FromForm, Request};

#[derive(Insertable, FromForm, Debug, Clone)]
#[table_name = "post"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
}
