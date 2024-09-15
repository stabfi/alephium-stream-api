use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod contracts;
pub mod events;

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Boolean(bool),
    Array(Arc<Vec<Val>>),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Val {
    r#type: String,
    value: Value,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct ApiErrorResponse {
    pub(crate) detail: String,
    pub(crate) resource: Option<String>,
}
