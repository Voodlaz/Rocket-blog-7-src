#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

mod db_conn;
mod schema;
mod posts;
use crate::posts::handlers;

use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn homepage() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("homepage", &context)
}

fn main() {
    rocket::ignite()
    .attach(Template::fairing())
    .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
    .mount("/", routes![homepage, handlers::new_post, handlers::new_post_form])
    .launch();
}
