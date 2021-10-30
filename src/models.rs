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