use diesel::Insertable;

use crate::schema::user_table;

#[derive(Insertable, Clone)]
#[table_name="user_table"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl NewUser {
    pub fn new(username: String, email: String, password: String) -> NewUser {
        return NewUser {
            username,
            email,
            password
        }
    }
}
