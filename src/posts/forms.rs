use crate::schema::posts;
use chrono::NaiveDateTime;

#[derive(FromForm, Insertable)]
#[table_name="posts"]
pub struct NewPostForm {
    pub name: String,
    pub body: String
}


#[derive(Debug, Queryable, Serialize)]
pub struct Post {
    pub name: String,
    pub body: String,
    pub creation_date: Option<NaiveDateTime>,
    pub id: i32
}
