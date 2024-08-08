use crate::error::ServiceError;
use futures::TryStreamExt;
use model::stream::Stream;
use mongodb::bson::doc;
use mongodb::Collection;

#[derive(Debug, Copy)]
pub enum StreamRole {
    Creator,
    Recipient,
}

pub trait StreamService {
    async fn get(&self, stream_id: &str) -> Result<Option<Stream>, ServiceError>;
    async fn list(&self, address: &str, role: StreamRole) -> Result<Vec<Stream>, ServiceError>;
}

pub struct StreamServiceImpl {
    pub collection: Collection<Stream>,
}

impl StreamService for StreamServiceImpl {
    async fn get(&self, stream_id: &str) -> Result<Stream, ServiceError> {
        let filter = doc! { "stream_id": stream_id };
        let stream = self
            .collection
            .find_one(filter)
            .await?
            .ok_or(ServiceError::NotFound)?;
        Ok(stream)
    }

    async fn list(&self, address: &str, role: StreamRole) -> Result<Vec<Stream>, ServiceError> {
        let filter = match role {
            StreamRole::Creator => doc! { "creator": address },
            StreamRole::Recipient => doc! { "recipient": address },
        };
        let streams = self.collection.find(filter).await?;
        Ok(streams.try_collect().await?)
    }
}
