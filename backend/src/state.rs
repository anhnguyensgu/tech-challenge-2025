use redis::Client;
use sqlx::PgPool;
use web3::{transports::Http, Web3};

use crate::{block::CacheStorage, database, gas::GasPriceStorage};

#[derive(Debug, Clone)]
pub struct AppState<GS: GasPriceStorage, BS: CacheStorage> {
    pub web3_client: Web3<Http>,
    pub pg_pool: PgPool,
    pub redis_client: Client,
    pub gas_price_storage: GS,
    pub block_price_storage: BS,
}

pub async fn new_web3_client() -> Web3<Http> {
    let transport =
        web3::transports::Http::new("https://ethereum-sepolia-rpc.publicnode.com").unwrap();
    web3::Web3::new(transport)
}

impl<GS: GasPriceStorage, BS: CacheStorage> AppState<GS, BS> {
    pub async fn new(gas: GS, block: BS, web3_client: Web3<Http>, redis_client: Client) -> Self {
        AppState {
            web3_client,
            pg_pool: database::init_pg_pool().await,
            redis_client,
            gas_price_storage: gas,
            block_price_storage: block,
        }
    }
}
