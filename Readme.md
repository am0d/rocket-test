## Rocket-test
A simple application to investigate what would work for an good app architecture.

Goals for this application:
- Should support multi-tenancy
  - Multi tenancy should be enforced in the type system
    - Ensure that every read / write to a table includes the tenant id of the current user
    - All database operations in the model implementations should require a connection that
      has the tenant id attached to it
    - Use a trait to automatically add the tenant id to each query before it runs, without
      having to explicitly add it in the code? e.g.
      ```
      impl Tenant for schema::period {
          type TenantID = schema::period::tenant_id;
      }

      impl Period {
          ...
          pub fn get(id: int, conn: &TenantConnection) -> AppError<Option<Period>> {
              period::table
                .filter(period::id.eq(id))
                // This line below should be inserted by the call to .first(...) below
                //.filter(period::tenant_id.eq(conn.tenant_id)) 
                .first::<Period>(conn)
                .map_err(|e| app_error!(DatabaseError, e))
          }
      }
      ```
    -  Should not be possible to query a tenanted table without filtering on tenant id