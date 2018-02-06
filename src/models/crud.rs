use diesel::pg::PgConnection;
use util::*;

pub trait Crud {
    fn get(id: i32, conn: &PgConnection) -> AppResult<Self>
    where
        Self: Sized;

    fn list(conn: &PgConnection) -> AppResult<Vec<Self>>
    where
        Self: Sized;
    fn save(&self, conn: &PgConnection) -> AppResult<Self>
    where
        Self: Sized;

    fn is_new(&self) -> bool;

    fn insert(&self, conn: &PgConnection) -> AppResult<Self>
    where
        Self: Sized;
    fn update(&self, conn: &PgConnection) -> AppResult<Self>
    where
        Self: Sized;
}
