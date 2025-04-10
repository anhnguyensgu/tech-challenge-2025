use crate::error::AppError;
use redis::{aio::MultiplexedConnection, AsyncCommands};

#[allow(async_fn_in_trait)]
pub trait CacheStorage {
    type Error;
    fn key() -> String;
    async fn get(&self) -> Result<u64, Self::Error>;
    async fn on_cache_miss(
        &self,
        con: &mut MultiplexedConnection,
    ) -> Result<u64, Self::Error>;
}

pub struct CachableBlockStorage {
    pub redis_client: redis::Client,
    pub web3_client: web3::Web3<web3::transports::Http>,
}

pub enum BlockStorageError {
    RedisError(redis::RedisError),
    Web3Error(web3::Error),
}

impl From<redis::RedisError> for BlockStorageError {
    fn from(err: redis::RedisError) -> Self {
        BlockStorageError::RedisError(err)
    }
}

impl From<web3::Error> for BlockStorageError {
    fn from(err: web3::Error) -> Self {
        BlockStorageError::Web3Error(err)
    }
}

impl From<BlockStorageError> for AppError {
    fn from(err: BlockStorageError) -> Self {
        match err {
            BlockStorageError::RedisError(e) => AppError::UnknownError(e.to_string()),
            BlockStorageError::Web3Error(e) => AppError::UnknownError(e.to_string()),
        }
    }
}

impl CacheStorage for CachableBlockStorage {
    type Error = BlockStorageError;

    fn key() -> String {
        "block_number".to_string()
    }

    async fn get(&self) -> Result<u64, Self::Error> {
        let mut con = self.redis_client.get_multiplexed_tokio_connection().await?;
        let cache_data: Option<u64> = con.get(Self::key()).await?;
        match cache_data {
            Some(price) => Ok(price),
            None => self.on_cache_miss(&mut con).await,
        }
    }

    async fn on_cache_miss(
        &self,
        con: &mut MultiplexedConnection,
    ) -> Result<u64, Self::Error> {
        let block_number = self.web3_client.eth().gas_price().await?.as_u64();
        let _: () = con.set_ex(Self::key(), block_number, 20).await?;
        Ok(block_number)
    }
}
