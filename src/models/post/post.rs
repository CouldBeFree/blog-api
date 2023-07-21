use crate::schema::post;
use super::super::user::user::User;
use chrono::NaiveDateTime;
// use diesel::{Queryable, Identifiable};

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(User)]
#[table_name="post"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub date: NaiveDateTime,
    pub user_id: i32
}