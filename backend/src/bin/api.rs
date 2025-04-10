use std::sync::Arc;

use axum::{routing::get, Router};
use backend::{account, database, state::AppState};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use web3::{transports::Http, Web3};

async fn new_web3_client() -> Web3<Http> {
    let transport =
        web3::transports::Http::new("https://ethereum-sepolia-rpc.publicnode.com").unwrap();
    web3::Web3::new(transport)
}

#[tokio::main]
async fn main() {
    let app = Arc::new(AppState {
        web3_client: new_web3_client().await,
        pg_pool: database::init_pg_pool().await,
    });

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router = Router::new()
        .route("/accounts", get(account::route::get_account_info))
        .with_state(app);
    let port = 3000;

    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("should create listener");
    info!("Listening on {port}");

    axum::serve(listener, router).await.unwrap();
}
