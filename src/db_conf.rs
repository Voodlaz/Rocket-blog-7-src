use rocket_contrib::databases::diesel;

#[database("prettycode_blog_db")]
pub struct DbConf(diesel::PgConnection);
