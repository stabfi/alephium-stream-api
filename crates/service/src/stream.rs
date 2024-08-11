use crate::error::ServiceError;
use futures::TryStreamExt;
use model::stream::Stream;
use mongodb::bson::doc;
use mongodb::Collection;

#[derive(Debug, Clone, Copy)]
pub enum StreamRole {
    Creator,
    Recipient,
}

#[async_trait::async_trait]
pub trait StreamService {
    async fn get(&self, stream_id: u32) -> Result<Stream, ServiceError>;
    async fn list(
        &self,
        address: &str,
        role: StreamRole,
        skip: u64,
    ) -> Result<Vec<Stream>, ServiceError>;
}

pub struct StreamServiceImpl {
    pub collection: Collection<Stream>,
}

#[async_trait::async_trait]
impl StreamService for StreamServiceImpl {
    async fn get(&self, stream_id: u32) -> Result<Stream, ServiceError> {
        let filter = doc! { "stream_id": stream_id };
        let stream = self
            .collection
            .find_one(filter)
            .await?
            .ok_or(ServiceError::NotFound)?;
        Ok(stream)
    }

    async fn list(
        &self,
        address: &str,
        role: StreamRole,
        skip: u64,
    ) -> Result<Vec<Stream>, ServiceError> {
        let filter = match role {
            StreamRole::Creator => doc! { "creator": address },
            StreamRole::Recipient => doc! { "recipient": address },
        };
        let streams = self.collection.find(filter).skip(skip).await?;
        Ok(streams.try_collect().await?)
    }
}
