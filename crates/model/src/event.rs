use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventField {
    pub r#type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub block_hash: String,
    pub transaction_id: String,
    pub event_index: u32,
    pub fields: Vec<EventField>,
    pub created_at: DateTime<Utc>,
}
