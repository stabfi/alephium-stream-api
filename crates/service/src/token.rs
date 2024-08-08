use crate::error::ServiceError;
use futures::TryStreamExt;
use model::token::Token;
use mongodb::bson::{doc, Bson};
use mongodb::Collection;

pub trait TokenService {
    async fn new(&self, token: Token) -> Result<Bson, ServiceError>;
    async fn get(&self, token_id: &str) -> Result<Token, ServiceError>;
    async fn list(&self) -> Result<Vec<Token>, ServiceError>;
}

pub struct TokenServiceImpl {
    pub collection: Collection<Token>,
}

impl TokenService for TokenServiceImpl {
    async fn new(&self, token: Token) -> Result<Bson, ServiceError> {
        let result = self.collection.insert_one(token).await?;
        Ok(result.inserted_id)
    }

    async fn get(&self, token_id: &str) -> Result<Token, ServiceError> {
        let filter = doc! { "token_id": token_id };
        let token = self
            .collection
            .find_one(filter)
            .await?
            .ok_or(ServiceError::NotFound)?;
        Ok(token)
    }

    async fn list(&self) -> Result<Vec<Token>, ServiceError> {
        let filter = doc! { "image_url": doc! { "$exists": true } };
        let tokens = self.collection.find(filter).await?;
        Ok(tokens.try_collect().await?)
    }
}
