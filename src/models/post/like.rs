use crate::schema::like;
use super::super::user::user::User;
// use super::super::post::post::Post;
use diesel::sql_types::Bool;

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Bool as SqlBool;
use std::io::Write;

// Newtype wrapper for bool
#[derive(Debug, Clone, Copy, PartialEq, FromSqlRow)]
pub struct CustomBool(bool);

impl FromSql<SqlBool, Pg> for CustomBool {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match bytes {
            Some(bytes) => Ok(CustomBool(bytes[0] != 0)),
            None => Ok(CustomBool(false)), // Change this default value if necessary
        }
    }
}

impl ToSql<SqlBool, Pg> for CustomBool {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        ToSql::<diesel::sql_types::Integer, Pg>::to_sql(&(self.0 as i32), out)
    }
}

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(User, Post)]
#[table_name="like"]
pub struct Like {
    pub id: i32,
    pub post_id: i32,
    pub user_id: i32,
    pub reaction: bool,
}