use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Post {
    #[validate(length(min = 4, max = 50))]
    pub title: String,
    #[validate(length(min = 5, max = 8000))]
    pub content: String
}
