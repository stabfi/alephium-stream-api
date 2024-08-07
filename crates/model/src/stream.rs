use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Active,
    Canceled,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StreamKind {
    Linear,
    Interval,
    Custom,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub kind: StreamKind,
    pub is_cancelable: bool,
    pub is_transferable: bool,

    pub start_timestamp: u32,
    pub end_timestamp: u32,

    pub unlock_interval: String,
    pub unlock_percentage: String,
    pub unlock_steps: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stream {
    pub id: u32,
    pub status: String,

    pub creator: String,
    pub recipient: String,

    pub amount: String,
    pub withdrawn_amount: String,

    pub token_id: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
