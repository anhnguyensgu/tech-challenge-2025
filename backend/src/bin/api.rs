use std::{env, sync::Arc};

use axum::{routing::get, Router};
use backend::{
    account,
    block::CachableBlockStorage,
    cache,
    gas::CachableGasPriceStorage,
    state::{new_web3_client, AppState},
    token,
};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let web3_client = new_web3_client().await;
    let redis_client = cache::init().await;
    let gas = CachableGasPriceStorage {
        redis_client: redis_client.clone(),
        web3_client: web3_client.clone(),
    };

    let block = CachableBlockStorage {
        redis_client: redis_client.clone(),
        web3_client: web3_client.clone(),
    };
    let app = Arc::new(AppState::new(gas, block, web3_client, redis_client).await);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router = Router::new()
        .route("/accounts", get(account::route::get_account_info))
        .route("/tokens", get(token::get_tokens_by_owner))
        .route("/tokens/{token_id}", get(token::get_token_by_id))
        .with_state(app);
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap();

    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("should create listener");
    info!("Listening on {port}");

    axum::serve(listener, router).await.unwrap();
}
