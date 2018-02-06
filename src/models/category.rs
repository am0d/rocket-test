use schema::category;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use util::*;

#[derive(Identifiable, Insertable, FromForm, Debug, Clone, AsChangeset, Queryable, Serialize,
         Default)]
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

    pub fn get(id: i32, conn: &PgConnection) -> AppResult<Category> {
        category::table
            .filter(category::id.eq(id))
            .first::<Category>(conn)
            .map_err(|e| app_error!(DatabaseError, e))
    }

    pub fn list(conn: &PgConnection) -> AppResult<Vec<Category>> {
        category::table
            .order(category::id)
            .load::<Category>(conn)
            .map_err(|e| app_error!(DatabaseError, e))
    }

    pub fn save(&self, conn: &PgConnection) -> AppResult<Category> {
        if self.is_new() {
            self.insert(conn)
        } else {
            self.update(conn)
        }
    }

    pub fn is_new(&self) -> bool {
        self.id == 0
    }

    fn insert(&self, conn: &PgConnection) -> AppResult<Category> {
        diesel::insert_into(category::table)
            .values(self)
            .get_result(conn)
            .map_err(|e| app_error!(DatabaseError, e))
    }

    fn update(&self, conn: &PgConnection) -> AppResult<Category> {
        use diesel::SaveChangesDsl;
        self.save_changes::<Category>(conn)
            .map_err(|e| app_error!(DatabaseError, e))
    }
}