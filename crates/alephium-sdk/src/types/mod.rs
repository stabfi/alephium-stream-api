use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct ApiErrorResponse {
    pub(crate) detail: String,
    pub(crate) resource: Option<String>,
}
