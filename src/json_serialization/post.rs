use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::NaiveDateTime;

#[derive(Deserialize, Validate, Serialize, Debug)]
pub struct Post {
    pub date: NaiveDateTime,
    pub id: i32,
    #[validate(length(min = 4, max = 50))]
    pub title: String,
    #[validate(length(min = 5, max = 8000))]
    pub content: String
}
