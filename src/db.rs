
//! Database connection helper functions
use std::ops::Deref;

use diesel::r2d2;
use diesel::r2d2::{Pool,PooledConnection,ConnectionManager};
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

pub type PgSqlPool = r2d2::Pool<ConnectionManager<PgConnection>>;


pub fn establish_connection() -> PgSqlPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("Failed to create database pool.")
}

pub struct PgSqlConn(PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for PgSqlConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for PgSqlConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<PgSqlConn, ()> {
        let pool = match <State<PgSqlPool> as FromRequest>::from_request(request) {
            Outcome::Success(pool) => pool,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(_) => return Outcome::Forward(())
        };

        match pool.get() {
            Ok(conn) => Outcome::Success(PgSqlConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}