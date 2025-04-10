use redis::{aio::MultiplexedConnection, AsyncCommands};

use crate::error::AppError;

fn gas_price_key() -> String {
    "gas_price".to_string()
}

pub async fn set_gas_price(
    con: &mut MultiplexedConnection,
    gas_price: u64,
) -> Result<(), redis::RedisError> {
    let _: () = con.set_ex(gas_price_key(), gas_price, 20).await?;
    Ok(())
}

#[allow(async_fn_in_trait)]
pub trait GasPriceStorage {
    type Error : Into<AppError>;
    fn gas_price_key() -> String;
    async fn get_gas_price(&self) -> Result<u64, Self::Error>;
}

pub struct CachableGasPriceStorage {
    pub redis_client: redis::Client,
    pub web3_client: web3::Web3<web3::transports::Http>,
}

pub enum GasError {
    RedisError(redis::RedisError),
    Web3Error(web3::Error),
}

impl From<redis::RedisError> for GasError {
    fn from(err: redis::RedisError) -> Self {
        GasError::RedisError(err)
    }
}

impl From<web3::Error> for GasError {
    fn from(err: web3::Error) -> Self {
        GasError::Web3Error(err)
    }
}

impl From<GasError> for AppError {
    fn from(err: GasError) -> Self {
        match err {
            GasError::RedisError(e) => AppError::UnknownError(e.to_string()),
            GasError::Web3Error(e) => AppError::UnknownError(e.to_string()),
        }
    }
}

impl GasPriceStorage for CachableGasPriceStorage {
    type Error = GasError;

    fn gas_price_key() -> String {
        gas_price_key()
    }
    async fn get_gas_price(&self) -> Result<u64, Self::Error> {
        let mut con = self.redis_client.get_multiplexed_tokio_connection().await?;
        let gas_price: Option<u64> = con.get(gas_price_key()).await?;
        match gas_price {
            Some(price) => Ok(price),
            None => {
                let gas_price = self.web3_client.eth().gas_price().await?.as_u64();
                set_gas_price(&mut con, gas_price).await?;
                Ok(gas_price)
            }
        }
    }
}
