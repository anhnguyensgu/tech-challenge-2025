use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct BlockOffset {
    pub current_offset: i64,
}

pub async fn current_offset(
    pool: &PgPool,
    contract_address: &str,
) -> Result<Option<BlockOffset>, sqlx::Error> {
    let offset =
        sqlx::query_as::<_, BlockOffset>("SELECT current_offset FROM block_offsets where address = $1")
            .bind(contract_address)
            .fetch_optional(pool)
            .await?;

    Ok(offset)
}

pub async fn update_offset(
    pool: &PgPool,
    contract_address: &str,
    new_offset: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO block_offsets (current_offset, address) VALUES ($1, $2) ON CONFLICT (address) DO UPDATE SET current_offset = $1")
        .bind(new_offset)
        .bind(contract_address)
        .execute(pool)
        .await?;

    Ok(())
}
