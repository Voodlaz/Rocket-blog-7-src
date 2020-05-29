use crate::posts::forms::{NewPostForm, Post};
use crate::db_conn::DbConn;
use crate::schema::posts;

use rocket_contrib::templates::Template;
use std::collections::HashMap;

use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};

use crate::diesel::RunQueryDsl;

use crate::diesel::query_dsl::filter_dsl::FindDsl;
use tera::Context;

#[get("/new_post")]
pub fn new_post<'signal>(signal: Option<FlashMessage<'_,'_>>) -> Template {
    dbg!(&signal);
    let mut context: HashMap<&str, String> = HashMap::new();
    if let Some(signal) = signal {
        let signal_error = signal.msg();
        context.insert("validation_error".into(), signal_error.into());
    }
    Template::render("new_post", context)
}

#[post("/new_post", data="<form>")]
pub fn new_post_form(form: Form<NewPostForm>, conn: DbConn) -> Flash<Redirect> {
    let form = form.into_inner();

    if form.name != "" && form.body != "" {
        diesel::insert_into(posts::table)
        .values(&form)
        .execute(&*conn)
        .expect("Can't create new post!");

        Flash::success(Redirect::to("/new_post"), "all working")
    } else {
        Flash::error(Redirect::to("/new_post"), "Server error 500")
    }
}

#[get("/post/<id>")]
pub fn post_handler(id: i32, conn: DbConn) -> Template {
    let mut context = Context::new();
    let post = posts::table.find(id).load::<Post>(&*conn).unwrap();

    context.insert("posts", &post);
    Template::render("post_handler", context)
}
