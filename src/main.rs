#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

mod db_conf;
mod schema;
mod posts;
use crate::posts::handlers;


use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket_contrib::serve::StaticFiles;

use dotenv::dotenv;
use std::env;

#[get("/")]
fn homepage() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("homepage", &context)
}

fn main() {
    dotenv().ok();
    rocket::ignite()
    .attach(db_conf::DbConf::fairing())
    .attach(Template::fairing())
    .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
    .mount("/", routes![homepage, handlers::new_post, handlers::new_post_form])
    .launch();
}
