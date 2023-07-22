use diesel::Insertable;

use crate::schema::post;
use chrono::{NaiveDateTime, Utc};

#[derive(Insertable, Clone)]
#[table_name="post"]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub user_id: i32,
    pub date: NaiveDateTime,
}

impl NewPost {
    pub fn new(title: String, content: String, user_id: i32) -> NewPost {
        let now = Utc::now().naive_local();
        return NewPost {
            title,
            content,
            date: now,
            user_id
        }
    }
}
