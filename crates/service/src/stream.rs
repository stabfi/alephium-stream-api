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
    async fn get_one(&self, stream_hash: String) -> Result<Stream, ServiceError>;
    async fn get_all(
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
    #[tracing::instrument(name = "stream_service::get_one", skip(self))]
    async fn get_one(&self, stream_hash: String) -> Result<Stream, ServiceError> {
        let filter = doc! { "hash": stream_hash };
        let stream = self
            .collection
            .find_one(filter)
            .await?
            .ok_or(ServiceError::NotFound)?;
        Ok(stream)
    }

    #[tracing::instrument(name = "stream_service::get_all", skip(self))]
    async fn get_all(
        &self,
        address: &str,
        role: StreamRole,
        skip: u64,
    ) -> Result<Vec<Stream>, ServiceError> {
        let filter = match role {
            StreamRole::Creator => doc! { "creator": address },
            StreamRole::Recipient => doc! { "recipient": address },
        };
        let streams = self
            .collection
            .find(filter)
            .skip(skip)
            .await?
            .try_collect()
            .await?;
        Ok(streams)
    }
}
