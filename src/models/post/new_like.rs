use crate::schema::like;

#[derive(Insertable, Clone)]
#[table_name="like"]
pub struct NewLike {
    pub reaction: bool,
    pub post_id: i32,
    pub user_id: i32
}

impl NewLike {
    pub fn new(reaction: bool, post_id: i32, user_id: i32) -> NewLike {
        return NewLike { 
            reaction,
            post_id,
            user_id
         }
    }
}