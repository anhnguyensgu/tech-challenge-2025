use sqlx::PgPool;
use web3::{transports::Http, Web3};

#[derive(Debug, Clone)]
pub struct AppState {
    pub web3_client: Web3<Http>,
    pub pg_pool: PgPool,
}
