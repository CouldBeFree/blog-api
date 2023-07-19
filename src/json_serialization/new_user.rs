use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct NewUserSchema {
    #[validate(length(min = 4, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String
}