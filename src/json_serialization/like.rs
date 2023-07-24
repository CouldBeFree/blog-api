use serde::Deserialize;

#[derive(Deserialize)]
pub struct Like {
    pub reaction: bool,
}
