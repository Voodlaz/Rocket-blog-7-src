use diesel::PgConnection;

#[database("prettycode_blog_db")]
pub struct DbConn(PgConnection);
