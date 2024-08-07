use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token_id: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u32,
    pub image_url: Option<String>,
}
