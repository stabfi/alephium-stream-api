use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Active,
    Canceled,
    Completed,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StreamKind {
    Linear,
    Interval,
    Custom,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnlockStep {
    pub timestamp: u64,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub kind: StreamKind,
    pub is_cancelable: bool,
    pub is_transferable: bool,

    pub start_timestamp: u64,
    pub end_timestamp: u64,

    pub unlock_interval: String,
    pub unlock_percentage: String,

    pub unlock_steps: Option<Vec<UnlockStep>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stream {
    pub id: u64,
    pub hash: String,

    pub status: Status,

    pub creator: String,
    pub recipient: String,

    pub amount: String,
    pub withdrawn_amount: String,

    pub token_id: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
