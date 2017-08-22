use schema::post;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
// use rocket::request::{self, FromForm, Request};

#[derive(Identifiable, Insertable, FromForm, Debug, Clone, AsChangeset, Queryable, Serialize)]
#[table_name = "post"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl Post {
    pub fn new() -> Post {
        Post {
            id: 0,
            title: "".to_string(),
            body: "".to_string(),
            published: false,
        }
    }

    pub fn get(id: i32, conn: &PgConnection) -> Post {
        post::table
            .filter(post::id.eq(id))
            .first::<Post>(conn)
            .unwrap()
    }

    pub fn list(conn: &PgConnection) -> Vec<Post> {
        post::table.order(post::id).load::<Post>(conn).unwrap()
    }

    pub fn save(&self, conn: &PgConnection) -> bool {
        if self.is_new() {
            self.insert(conn)
        } else {
            self.update(conn)
        }
    }

    pub fn is_new(&self) -> bool {
        self.id == 0
    }

    fn insert(&self, conn: &PgConnection) -> bool {
        diesel::insert(self).into(post::table).execute(conn).is_ok()
    }

    fn update(&self, conn: &PgConnection) -> bool {
        use diesel::SaveChangesDsl;
        self.save_changes::<Post>(conn).is_ok()
        //diesel::update(post::table).set(self).execute(conn).is_ok()
    }

}