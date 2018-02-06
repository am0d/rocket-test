#[macro_export]
macro_rules! impl_crud {
    ($table: ident, $table_type: ident) => {
        impl Crud for $table {

            fn get(id: i32, conn: &PgConnection) -> AppResult<Self>
            {
                $table_type::table
                    .filter($table_type::id.eq(id))
                    .first::<Self>(conn)
                    .map_err(|e| app_error!(DatabaseError, e))
            }

            fn list(conn: &PgConnection) -> AppResult<Vec<Self>>
            where
                Self: Sized,
            {
                $table_type::table
                    .order($table_type::id)
                    .load::<Self>(conn)
                    .map_err(|e| app_error!(DatabaseError, e))
            }

            fn save(&self, conn: &PgConnection) -> AppResult<Self>
            where
                Self: Sized,
            {
                if self.is_new() {
                    self.insert(conn)
                } else {
                    self.update(conn)
                }
            }

            fn is_new(&self) -> bool {
                self.id == 0
            }

            fn insert(&self, conn: &PgConnection) -> AppResult<Self>
            where
                Self: Sized,
            {
                diesel::insert_into($table_type::table)
                    .values(self)
                    .get_result(conn)
                    .map_err(|e| app_error!(DatabaseError, e))
            }

            fn update(&self, conn: &PgConnection) -> AppResult<Self>
            where
                Self: Sized,
            {
                use diesel::SaveChangesDsl;
                self.save_changes::<Self>(conn)
                    .map_err(|e| app_error!(DatabaseError, e))
            }
        }
    }
}
