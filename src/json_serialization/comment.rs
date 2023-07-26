use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::NaiveDateTime;

#[derive(Deserialize, Validate, Serialize, Debug)]
pub struct Comment {
    pub date: Option<NaiveDateTime>,
    pub id: Option<i32>,
    #[validate(length(min = 5, max = 8000))]
    pub content: String
}
