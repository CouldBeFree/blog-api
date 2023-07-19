extern crate bcrypt;

use diesel::{Queryable, Identifiable};
use bcrypt::verify;

use crate::schema::user_table;

#[derive(Queryable, Identifiable, Clone)]
#[table_name="user_table"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn verify(&self, password: String) -> bool {
        verify(password.as_str(), &self.password).unwrap()
    }
}