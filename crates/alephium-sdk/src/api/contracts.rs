use crate::client::ApiClient;
use crate::types::contracts::{
    ContractCallBody, ContractCallResponse, MultipleCallContractBody, MultipleCallContractResponse,
};
use reqwest::Method;

#[async_trait::async_trait]
pub trait ContractsGroup {
    async fn call_contract(&self, body: ContractCallBody)
        -> crate::ApiResult<ContractCallResponse>;

    async fn multi_call_contract(
        &self,
        body: MultipleCallContractBody,
    ) -> crate::ApiResult<MultipleCallContractResponse>;
}

#[async_trait::async_trait]
impl ContractsGroup for ApiClient {
    async fn call_contract(&self, body: ContractCallBody) -> crate::ApiResult<ContractCallResponse> {
        self.request(
            Method::POST,
            "contracts/call-contract".to_string(),
            None::<String>,
            Some(body),
        )
        .await
    }

    async fn multi_call_contract(
        &self,
        body: MultipleCallContractBody,
    ) -> crate::ApiResult<MultipleCallContractResponse> {
        self.request(
            Method::POST,
            "contracts/multicall-contract".to_string(),
            None::<String>,
            Some(body),
        )
        .await
    }
}
