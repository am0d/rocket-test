use schema::post;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

#[derive(Insertable, Debug, Clone, Serialize)]
#[table_name = "post"]
struct NewPost<'a> {
    title: &'a str,
    body: &'a str,
    published: bool,
}

#[derive(Identifiable, Insertable, FromForm, Debug, Clone, AsChangeset, Queryable, Serialize,
         Default)]
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
            NewPost::from(self).insert(conn)
        } else {
            self.update(conn)
        }
    }

    pub fn is_new(&self) -> bool {
        self.id == 0
    }

    fn update(&self, conn: &PgConnection) -> bool {
        use diesel::SaveChangesDsl;
        self.save_changes::<Post>(conn).is_ok()
    }
}

impl<'a> NewPost<'a> {
    fn from(post: &'a Post) -> NewPost<'a> {
        NewPost {
            title: &post.title,
            body: &post.body,
            published: post.published,
        }
    }
    fn insert(&self, conn: &PgConnection) -> bool {
        diesel::insert_into(post::table)
            .values(self)
            .execute(conn)
            .is_ok()
    }
}
