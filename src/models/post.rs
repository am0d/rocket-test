use schema::post;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
// use rocket::request::{self, FromForm, Request};

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
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl Post {
    pub fn get(id: i32, conn: &PgConnection) -> Post {
        post::table.filter(post::id.eq(id)).first::<Post>(conn).unwrap()
    }

    pub fn list(conn: &PgConnection) -> Vec<Post> {
        post::table.order(post::id).load::<Post>(conn).unwrap()
    }
}