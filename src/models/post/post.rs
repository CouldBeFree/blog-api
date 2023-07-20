use crate::schema::post;
use super::super::user::user::User;

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(User)]
#[table_name="post"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32
}
