#[macro_export]
macro_rules! impl_crud {
    ($table: ident, $new_type: ident, $table_type: ident) => {
        impl Crud for $table {
            type Table = $table_type::table;
            type Id = $table_type::id;
            type NewRecord = $new_type;

            fn get(id: i32, conn: &PgConnection) -> AppResult<Self>
                where Self:Sized,
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

            fn is_new(&self) -> bool {
                self.id == 0
            }

            fn save(&self, conn: &PgConnection) -> AppResult<Self>
            where
                Self: Sized,
            {
                if self.is_new() {
                    $new_type::from(self).insert(conn)
                } else {
                    self.update(conn)
                }
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
