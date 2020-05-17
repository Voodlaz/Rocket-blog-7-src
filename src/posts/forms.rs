use crate::schema::posts;
use chrono::NaiveDateTime;

#[derive(FromForm, Debug, Insertable)]
#[table_name="posts"]
pub struct NewPostForm {
    pub name: String,
    pub body: String
}

#[derive(Debug, Queryable)]
pub struct Post {
    pub name: String,
    pub body: String,
    pub id: i32,
    pub creation_date: NaiveDateTime
}
