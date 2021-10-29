use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Url {
    pub url: url::Url,
}