/// Defines the database schema.

mod db_schema;
pub use self::db_schema::*;
/*
use super::db::PgSqlConn;
use diesel::query_dsl::*;
use diesel::QueryResult;
use diesel::dsl::Limit;
use diesel;

pub trait HasTenantID {
    type TenantIDColumn;
}

pub trait NoTenantID {

}


impl HasTenantID for db_schema::period::table {
    type TenantIDColumn = db_schema::period::id;
}

pub struct TenantedConnection {
    connection: PgSqlConn,
    tenant_id: u32
}

/// Methods used to execute queries.
pub trait RunTenantedQueryDsl: Sized {
    /// Executes the given command, returning the number of rows affected.
    ///
    /// Used in conjunction with [`insert_into`](../fn.insert_into.html),
    /// [`update`](../fn.update.html) and [`delete`](../fn.delete.html)
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[macro_use] extern crate diesel;
    /// # include!("../doctest_setup.rs");
    /// #
    /// # fn main() {
    /// #     run_test();
    /// # }
    /// #
    /// # fn run_test() -> QueryResult<()> {
    /// #     use diesel::insert_into;
    /// #     use schema::users::dsl::*;
    /// #     let connection = establish_connection();
    /// let inserted_rows = insert_into(users)
    ///     .values(name.eq("Ruby"))
    ///     .execute(&connection)?;
    /// assert_eq!(1, inserted_rows);
    ///
    /// let inserted_rows = insert_into(users)
    ///     .values(&vec![name.eq("Jim"), name.eq("James")])
    ///     .execute(&connection)?;
    /// assert_eq!(2, inserted_rows);
    /// #     Ok(())
    /// # }
    /// ```
    fn execute(self, conn: &TenantedConnection) -> QueryResult<usize>
    where
        Self: methods::ExecuteDsl<Connection>,
    {
        methods::ExecuteDsl::execute(self, &(conn.connection))
    }

    /// Executes the given query, returning a `Vec` with the returned rows.
    ///
    /// When using the query builder,
    /// the return type can be
    /// a tuple of the values,
    /// or a struct which implements [`Queryable`].
    ///
    /// When this method is called on [`sql_query`],
    /// the return type can only be a struct which implements [`QueryableByName`]
    ///
    /// [`Queryable`]: ../deserialize/trait.Queryable.html
    /// [`QueryableByName`]: ../deserialize/trait.QueryableByName.html
    /// [`sql_query`]: ../fn.sql_query.html
    ///
    /// # Examples
    ///
    /// ## Returning a single field
    ///
    /// ```rust
    /// # #[macro_use] extern crate diesel;
    /// # include!("../doctest_setup.rs");
    /// #
    /// # fn main() {
    /// #     run_test();
    /// # }
    /// #
    /// # fn run_test() -> QueryResult<()> {
    /// #     use diesel::insert_into;
    /// #     use schema::users::dsl::*;
    /// #     let connection = establish_connection();
    /// let data = users.select(name)
    ///     .load::<String>(&connection)?;
    /// assert_eq!(vec!["Sean", "Tess"], data);
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// ## Returning a tuple
    ///
    /// ```rust
    /// # #[macro_use] extern crate diesel;
    /// # include!("../doctest_setup.rs");
    /// #
    /// # fn main() {
    /// #     run_test();
    /// # }
    /// #
    /// # fn run_test() -> QueryResult<()> {
    /// #     use diesel::insert_into;
    /// #     use schema::users::dsl::*;
    /// #     let connection = establish_connection();
    /// let data = users
    ///     .load::<(i32, String)>(&connection)?;
    /// let expected_data = vec![
    ///     (1, String::from("Sean")),
    ///     (2, String::from("Tess")),
    /// ];
    /// assert_eq!(expected_data, data);
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// ## Returning a struct
    ///
    /// ```rust
    /// # #[macro_use] extern crate diesel;
    /// # include!("../doctest_setup.rs");
    /// #
    /// #[derive(Queryable, PartialEq, Debug)]
    /// struct User {
    ///     id: i32,
    ///     name: String,
    /// }
    ///
    /// # fn main() {
    /// #     run_test();
    /// # }
    /// #
    /// # fn run_test() -> QueryResult<()> {
    /// #     use diesel::insert_into;
    /// #     use schema::users::dsl::*;
    /// #     let connection = establish_connection();
    /// let data = users
    ///     .load::<User>(&connection)?;
    /// let expected_data = vec![
    ///     User { id: 1, name: String::from("Sean"), },
    ///     User { id: 2, name: String::from("Tess"), },
    /// ];
    /// assert_eq!(expected_data, data);
    /// #     Ok(())
    /// # }
    /// ```
    fn load<U>(self, conn: &TenantedConnection) -> QueryResult<Vec<U>>
    where
        Self: LoadQuery<TenantedConnection, U>,
    {
        self.internal_load(conn)
    }

    /// Runs the command, and returns the affected row.
    ///
    /// `Err(NotFound)` will be returned if the query affected 0 rows. You can
    /// call `.optional()` on the result of this if the command was optional to
    /// get back a `Result<Option<U>>`
    ///
    /// When this method is called on an insert, update, or delete statement,
    /// it will implicitly add a `RETURNING *` to the query,
    /// unless a returning clause was already specified.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[macro_use] extern crate diesel;
    /// # include!("../doctest_setup.rs");
    /// #
    /// # fn main() {
    /// #     run_test();
    /// # }
    /// #
    /// # #[cfg(feature = "postgres")]
    /// # fn run_test() -> QueryResult<()> {
    /// #     use diesel::{insert_into, update};
    /// #     use schema::users::dsl::*;
    /// #     let connection = establish_connection();
    /// let inserted_row = insert_into(users)
    ///     .values(name.eq("Ruby"))
    ///     .get_result(&connection)?;
    /// assert_eq!((3, String::from("Ruby")), inserted_row);
    ///
    /// // This will return `NotFound`, as there is no user with ID 4
    /// let update_result = update(users.find(4))
    ///     .set(name.eq("Jim"))
    ///     .get_result::<(i32, String)>(&connection);
    /// assert_eq!(Err(diesel::NotFound), update_result);
    /// #     Ok(())
    /// # }
    /// #
    /// # #[cfg(not(feature = "postgres"))]
    /// # fn run_test() -> QueryResult<()> {
    /// #     Ok(())
    /// # }
    /// ```
    fn get_result<U>(self, conn: &TenantedConnection) -> QueryResult<U>
    where
        Self: LoadQuery<TenantedConnection, U>,
    {
        //first_or_not_found(self.load(conn))
        self.load(conn)?.into_iter().next().ok_or(diesel::result::Error::NotFound)
    }

    /// Runs the command, returning an `Vec` with the affected rows.
    ///
    /// This method is an alias for [`load`], but with a name that makes more
    /// sense for insert, update, and delete statements.
    ///
    /// [`load`]: #method.load
    fn get_results<U>(self, conn: &TenantedConnection) -> QueryResult<Vec<U>>
    where
        Self: LoadQuery<TenantedConnection, U>,
    {
        self.load(conn)
    }

    /// Attempts to load a single record.
    ///
    /// This method is equivalent to `.limit(1).get_result()`
    ///
    /// Returns `Ok(record)` if found, and `Err(NotFound)` if no results are
    /// returned. If the query truly is optional, you can call `.optional()` on
    /// the result of this to get a `Result<Option<U>>`.
    ///
    /// # Example:
    ///
    /// ```rust
    /// # #[macro_use] extern crate diesel;
    /// # include!("../doctest_setup.rs");
    /// # fn main() {
    /// #     run_test();
    /// # }
    /// #
    /// # fn run_test() -> QueryResult<()> {
    /// #     use schema::users::dsl::*;
    /// #     let connection = establish_connection();
    /// diesel::insert_into(users)
    ///     .values(&vec![name.eq("Sean"), name.eq("Pascal")])
    ///     .execute(&connection)?;
    ///
    /// let first_name = users.order(id).select(name).first(&connection);
    /// assert_eq!(Ok(String::from("Sean")), first_name);
    ///
    /// let not_found = users
    ///     .filter(name.eq("Foo"))
    ///     .first::<(i32, String)>(&connection);
    /// assert_eq!(Err(diesel::NotFound), not_found);
    /// #     Ok(())
    /// # }
    /// ```
    fn first<U>(self, conn: &TenantedConnection) -> QueryResult<U>
    where
        Self: methods::LimitDsl,
        Limit<Self>: LoadQuery<TenantedConnection, U>,
    {
        methods::LimitDsl::limit(self, 1).get_result(conn)
    }
}
*/