use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Url {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShortUrl {
    pub id: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct Id {
    pub id: String,
}

#[derive(Serialize)]
pub struct ErrorMessage {
    pub msg: String,
}

impl ErrorMessage {
    pub fn new(s: &str) -> Self {
        Self {
            msg: s.to_string(),
        }
    }
}