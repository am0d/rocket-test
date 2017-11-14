use schema::category;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

#[derive(Insertable, Debug, Clone, Serialize)]
#[table_name = "category"]
struct NewCategory<'a> {
    name: &'a str,
}

#[derive(Identifiable, Insertable, FromForm, Debug, Clone, AsChangeset, Queryable, Serialize, Default)]
#[table_name = "category"]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(FromForm, Debug, Clone)]
pub struct CategoryForm {
    pub id: i32,
    pub name: String,
}

impl Category {
    pub fn new() -> Category {
        Category {
            id: 0,
            name: "".to_string(),
        }
    }

    pub fn get(id: i32, conn: &PgConnection) -> Category {
        category::table
            .filter(category::id.eq(id))
            .first::<Category>(conn)
            .unwrap()
    }

    pub fn list(conn: &PgConnection) -> Vec<Category> {
        category::table
            .order(category::id)
            .load::<Category>(conn)
            .unwrap()
    }

    pub fn save(&self, conn: &PgConnection) -> bool {
        if self.is_new() {
            NewCategory::from(self).insert(conn)
        } else {
            self.update(conn)
        }
    }

    pub fn is_new(&self) -> bool {
        self.id == 0
    }

    fn update(&self, conn: &PgConnection) -> bool {
        use diesel::SaveChangesDsl;
        self.save_changes::<Category>(conn).is_ok()
        //diesel::update(category::table).set(self).execute(conn).is_ok()
    }
}

impl<'a> NewCategory<'a> {
    fn from(category: &'a Category) -> NewCategory<'a> {
        NewCategory { name: &category.name }
    }
    fn insert(&self, conn: &PgConnection) -> bool {
        diesel::insert(self)
            .into(category::table)
            .execute(conn)
            .is_ok()
    }
}