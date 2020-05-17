use crate::posts::forms::{NewPostForm, Post};

use crate::db_conf::DbConf;
use crate::schema::posts;
use crate::diesel::RunQueryDsl;

use rocket_contrib::templates::Template;
use std::collections::HashMap;

use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};

#[get("/new_post")]
pub fn new_post<'signal>(signal: Option<FlashMessage<'_,'_>>, conn: DbConf) -> Template {
    dbg!(&signal);
    let mut context: HashMap<&str, String> = HashMap::new();

    let post_names = posts::table.load::<Post>(&*conn);
    context.insert("db-table-name", post_names.into());

    if let Some(signal) = signal {
        if signal.name() == "success" {
            let signal_success = signal.msg();
            context.insert("validation_ok".into(), signal_success.into());
        } else {
            let signal_error = signal.msg();
            context.insert("validation_error_error".into(), signal_error.into());}}
    Template::render("new_post", context)
}

#[post("/new_post", data="<form>")]
pub fn new_post_form(form: Form<NewPostForm>, conn: DbConf) -> Flash<Redirect> {
    let form = form.into_inner();

    if form.name != "" && form.body != "" {
        diesel::insert_into(posts::table)
        .values(&form)
        .execute(&*conn);

        Flash::success(Redirect::to("/new_post"), "all working")
    } else {
        Flash::error(Redirect::to("/new_post"), "Can't create new post!")
    }
}
