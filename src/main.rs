#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

mod db_conn;
mod schema;
mod posts;
use crate::posts::{handlers, forms};
use forms::Post;

use rocket_contrib::templates::Template;

use rocket_contrib::serve::StaticFiles;

use crate::schema::posts::dsl::*;
use diesel::RunQueryDsl;
use db_conn::DbConn;

use tera::Context;

#[get("/")]
fn homepage(conn: DbConn) -> Template {
    let mut context = Context::new();
    let all_posts = posts.load::<Post>(&*conn).unwrap();

    context.insert("posts", &all_posts);
    Template::render("homepage", &context)
}

fn main() {
    rocket::ignite()
    .attach(DbConn::fairing())
    .attach(Template::fairing())
    .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
    .mount("/", routes![homepage, handlers::post_handler, handlers::new_post, handlers::new_post_form])
    .launch();
}
