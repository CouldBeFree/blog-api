use diesel::Insertable;

use crate::schema::post;

#[derive(Insertable, Clone)]
#[table_name="post"]
pub struct NewPost {
    pub title: String,
    pub content: String,
}

impl NewPost {
    pub fn new(title: String, content: String, user_id: i32) -> NewPost {
        return NewPost {
            title,
            content,
        }
    }
}
