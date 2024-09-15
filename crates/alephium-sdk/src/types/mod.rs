use serde::{Deserialize, Serialize};

pub mod contracts;
pub mod events;

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Boolean(bool),
    Array(Vec<Val>),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Val {
    pub r#type: String,
    pub value: Value,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct ApiErrorResponse {
    pub(crate) detail: String,
    pub(crate) resource: Option<String>,
}
