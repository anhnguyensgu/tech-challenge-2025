use crate::{
    account::persistence::{account_by_address, upsert_account_info},
    error::AppError,
    response::AppJson,
    state::AppState,
};
use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use validator::{Validate, ValidationError};
use web3::types::Address;

use super::persistence::Account;

#[derive(Serialize, Debug)]
pub struct AccounInfo {
    pub address: String,
    pub balance: u64,
    pub block_number: u64,
    pub gas_price: u64,
}

impl From<Account> for AccounInfo {
    fn from(
        Account {
            address,
            balance,
            block_number,
        }: Account,
    ) -> Self {
        Self {
            address,
            balance: balance as u64,
            block_number: block_number as u64,
            gas_price: 0,
        }
    }
}

#[derive(Deserialize, Debug, validator::Validate)]
pub struct AccountInfoQuery {
    #[validate(custom(function = "validate_evm_address"))]
    pub address: String,
}

fn validate_evm_address(address: &str) -> Result<(), ValidationError> {
    if address.len() != 42 || !address.starts_with("0x") {
        return Err(ValidationError::new("Invalid length or missing 0x prefix"));
    }
    let addr = &address[2..];
    if !addr.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ValidationError::new("Contains non-hex characters"));
    }
    Ok(())
}

pub async fn get_account_info(
    Query(params): Query<AccountInfoQuery>,
    State(app): State<Arc<AppState>>,
) -> Result<AppJson<AccounInfo>, AppError> {
    let web3_client = &app.web3_client;
    if let Err(e) = params.validate() {
        return Err(AppError::UnknownError(e.to_string()));
    }

    let gas_price = web3_client.eth().gas_price().await?;
    let block_number = web3_client.eth().block_number().await?;
    info!("latest block number: {:?}", block_number);

    match account_by_address(&app.pg_pool, &params.address, block_number.as_u64() as i64).await {
        Ok(Some(account)) => {
            let mut account_info: AccounInfo = account.into();
            account_info.gas_price = gas_price.as_u64();
            Ok(AppJson(account_info))
        }
        Ok(None) => {
            info!(
                "account {} with block {} not found  in db",
                &params.address,
                block_number.as_u64()
            );
            let address = params.address.replace("0x", "");
            let address: Address = address.parse().unwrap();
            let balance = web3_client.eth().balance(address, None).await.unwrap();
            let Some(account) = upsert_account_info(
                &app.pg_pool,
                &params.address,
                balance.as_u64() as i64,
                block_number.as_u64() as i64,
            )
            .await?
            else {
                return Err(AppError::UnknownError(
                    "Failed to upsert account info".to_string(),
                ));
            };

            let mut account_info: AccounInfo = account.into();
            account_info.gas_price = gas_price.as_u64();
            Ok(AppJson(account_info))
        }
        Err(e) => {
            info!("error from get_account_info: {:?}", e);
            Err(AppError::UnknownError(e.to_string()))
        }
    }
}

impl From<web3::Error> for AppError {
    fn from(err: web3::Error) -> Self {
        AppError::UnknownError(err.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::UnknownError(err.to_string())
    }
}
