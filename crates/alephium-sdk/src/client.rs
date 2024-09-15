use crate::error::ApiError;
use crate::types::ApiErrorResponse;
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;

pub struct ApiClient {
    pub(crate) url: Url,
    pub(crate) client: Client,
}

impl ApiClient {
    pub fn new(url: Url) -> Self {
        Self {
            url,
            client: Default::default(),
        }
    }

    pub fn from_client(url: Url, client: Client) -> Self {
        Self { url, client }
    }

    pub(crate) async fn request<T: DeserializeOwned, Q: Serialize + Sized, J: Serialize + Sized>(
        &self,
        method: Method,
        path: String,
        query: Option<Q>,
        json: Option<J>,
    ) -> Result<T, ApiError> {
        let mut request = self
            .client
            .request(method, format!("{}{}", self.url.as_str(), path));

        if query.is_some() {
            request = request.query(&query.unwrap());
        }

        if json.is_some() {
            request = request.json(&json.unwrap());
        }

        let response = request.send().await?;

        if response.status().is_success() {
            Ok(response.json::<T>().await?)
        } else {
            Err(ApiError::from_error_response(
                response.status(),
                response.json::<ApiErrorResponse>().await?,
            ))
        }
    }
}
