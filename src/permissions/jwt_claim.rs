use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: u64,
    pub iss: String,
    pub username: String
}
