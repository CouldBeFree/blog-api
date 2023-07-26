use crate::schema::like;
use super::super::user::user::User;

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(User, Post)]
#[table_name="like"]
pub struct Like {
    pub id: i32,
    pub post_id: i32,
    pub user_id: i32,
    pub reaction: bool,
}