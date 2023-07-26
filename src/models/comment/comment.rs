use crate::schema::comment;
use super::super::user::user::User;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(User, Post)]
#[table_name="comment"]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub post_id: i32,
    pub content: String,
    pub date: NaiveDateTime,
}