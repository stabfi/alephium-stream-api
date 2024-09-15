use crate::client::ApiClient;
use crate::types::events::{ContractEventsQuery, ContractEventsResponse};
use reqwest::Method;

#[async_trait::async_trait]
pub trait EventsGroup {
    async fn get_contract_events(
        &self,
        contract_address: String,
        query: ContractEventsQuery,
    ) -> crate::ApiResult<ContractEventsResponse>;
}

#[async_trait::async_trait]
impl EventsGroup for ApiClient {
    async fn get_contract_events(
        &self,
        contract_address: String,
        query: ContractEventsQuery,
    ) -> crate::ApiResult<ContractEventsResponse> {
        self.request(
            Method::GET,
            format!("events/contract/{}", contract_address.as_str()),
            Some(query),
            None::<String>,
        )
        .await
    }
}
