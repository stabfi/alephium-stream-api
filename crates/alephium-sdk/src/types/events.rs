use crate::types::Val;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    #[serde(rename(deserialize = "blockHash"))]
    pub block_hash: String,
    #[serde(rename(deserialize = "txId"))]
    pub tx_id: String,
    #[serde(rename(deserialize = "eventIndex"))]
    pub event_index: u64,
    pub fields: Vec<Val>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ContractEventsResponse {
    pub events: Vec<Event>,
    #[serde(rename(deserialize = "nextStart"))]
    pub next_start: u64,
}

#[derive(Serialize, Debug)]
pub struct ContractEventsQuery {
    pub start: i32,
    pub limit: Option<i32>,
    pub group: Option<i32>,
}
