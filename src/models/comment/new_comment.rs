use crate::schema::comment;
use chrono::{NaiveDateTime, Utc};

#[derive(Insertable, Clone)]
#[table_name="comment"]
pub struct NewComment {
    pub user_id: i32,
    pub post_id: i32,
    pub content: String,
    pub date: NaiveDateTime,
}

impl NewComment {
    pub fn new(content: String, post_id: i32, user_id: i32) -> NewComment {
        let now = Utc::now().naive_local();
        return NewComment {
            post_id,
            user_id,
            date: now,
            content
         }
    }
}