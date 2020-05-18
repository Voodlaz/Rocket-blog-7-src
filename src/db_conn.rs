use diesel::PgConnection;

#[database("pg_db")]
pub struct DbConn(PgConnection);
