use crate::{
    block::CacheStorage, error::AppError, gas::GasPriceStorage, response::AppJson, state::AppState,
};
use axum::extract::{Path, Query, State};
use std::sync::Arc;

use super::{by_id, by_owner, TokenOwner};

#[derive(serde::Deserialize, Debug, validator::Validate)]
pub struct TokenOwnerQuery {
    pub owner_address: String,
}

pub async fn get_tokens_by_owner<GS: GasPriceStorage, BS: CacheStorage>(
    Query(params): Query<TokenOwnerQuery>,
    State(app): State<Arc<AppState<GS, BS>>>,
) -> Result<AppJson<Vec<TokenOwner>>, AppError>
where
    AppError: From<<GS as GasPriceStorage>::Error>,
    AppError: From<<BS as CacheStorage>::Error>,
{
    let TokenOwnerQuery { owner_address } = params;
    match by_owner(&app.pg_pool, &owner_address).await {
        Ok(token_owner) => Ok(AppJson(token_owner)),
        Err(e) => Err(AppError::UnknownError(e.to_string())),
    }
}

pub async fn get_token_by_id<GS: GasPriceStorage, BS: CacheStorage>(
    Path(token_id): Path<i32>,
    State(app): State<Arc<AppState<GS, BS>>>,
) -> Result<AppJson<TokenOwner>, AppError>
where
    AppError: From<<GS as GasPriceStorage>::Error>,
    AppError: From<<BS as CacheStorage>::Error>,
{
    match by_id(&app.pg_pool, token_id).await {
        Ok(Some(token_owner)) => Ok(AppJson(token_owner)),
        Ok(None) => Err(AppError::UnknownError(format!(
            "Token with id {token_id} not found"
        ))),
        Err(e) => Err(AppError::UnknownError(e.to_string())),
    }
}
