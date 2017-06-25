use db::{self, PgSqlConn};
use schema::post;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use rocket::request::{self, FromForm, Request};

#[derive(Insertable, FromForm, Debug, Clone)]
#[table_name = "post"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl NewPost {
    pub fn insert(&self, conn: &PgConnection) -> bool {
        diesel::insert(self).into(post::table).execute(conn).is_ok()
    }
}