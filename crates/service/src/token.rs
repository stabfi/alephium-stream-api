use crate::error::ServiceError;
use futures::TryStreamExt;
use model::token::Token;
use mongodb::bson::{doc, Bson};
use mongodb::Collection;

#[async_trait::async_trait]
pub trait TokenService {
    async fn new(&self, token: Token) -> Result<Bson, ServiceError>;
    async fn get_one(&self, token_id: &str) -> Result<Token, ServiceError>;
    async fn get_all(&self) -> Result<Vec<Token>, ServiceError>;
}

pub struct TokenServiceImpl {
    pub collection: Collection<Token>,
}

#[async_trait::async_trait]
impl TokenService for TokenServiceImpl {
    #[tracing::instrument(name = "create_token", skip(self))]
    async fn new(&self, token: Token) -> Result<Bson, ServiceError> {
        let result = self.collection.insert_one(token).await?;

        Ok(result.inserted_id)
    }

    #[tracing::instrument(name = "get_one_token", skip(self))]
    async fn get_one(&self, token_id: &str) -> Result<Token, ServiceError> {
        let filter = doc! { "token_id": token_id };
        let token = self
            .collection
            .find_one(filter)
            .await?
            .ok_or(ServiceError::NotFound)?;

        Ok(token)
    }

    #[tracing::instrument(name = "get_all_tokens", skip(self))]
    async fn get_all(&self) -> Result<Vec<Token>, ServiceError> {
        let filter = doc! { "image_url": doc! { "$exists": true } };
        let tokens = self.collection.find(filter).await?;

        Ok(tokens.try_collect().await?)
    }
}
