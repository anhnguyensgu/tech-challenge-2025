use serde::Serialize;
use sqlx::{prelude::FromRow, PgPool};
pub mod route;

#[derive(FromRow, Debug, Serialize)]
pub struct TokenOwner {
    pub token_id: i32,
    pub owner_id: String,
    pub block_number: i64,
}

pub async fn upsert_token_owner(
    pool: &PgPool,
    token_id: i32,
    owner_of: &str,
    block_number: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO token_owner (token_id, owner_id, block_number) 
        VALUES ($1, $2, $3) 
        ON CONFLICT (token_id) 
        DO UPDATE SET owner_id = EXCLUDED.owner_id, block_number = EXCLUDED.block_number 
        WHERE token_owner.block_number <= EXCLUDED.block_number"#,
    )
    .bind(token_id)
    .bind(owner_of)
    .bind(block_number)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn by_owner(pool: &PgPool, owner_id: &str) -> Result<Vec<TokenOwner>, sqlx::Error> {
    sqlx::query_as::<_, TokenOwner>(
        r#"
        SELECT token_id, owner_id, block_number from token_owner
        WHERE owner_id = $1
        "#,
    )
    .bind(owner_id)
    .fetch_all(pool)
    .await
}

pub async fn by_id(pool: &PgPool, token_id: i32) -> Result<Option<TokenOwner>, sqlx::Error> {
    sqlx::query_as::<_, TokenOwner>(
        r#"
        SELECT token_id, owner_id, block_number from token_owner
        WHERE token_id = $1
        "#,
    )
    .bind(token_id)
    .fetch_optional(pool)
    .await
}

pub use route::*;
