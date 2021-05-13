use crate::schema::*;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}
