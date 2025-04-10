use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Account {
    pub address: String,
    pub balance: i64,
    pub block_number: i64,
    //pub created_at: u64,
    //pub updated_at: u64,
}

pub async fn account_by_address(
    pool: &sqlx::PgPool,
    address: &str,
    block_number: i64,
) -> Result<Option<Account>, sqlx::Error> {
    let account = sqlx::query_as::<_, Account>(
        r#"
        SELECT address, balance, block_number, created_at, updated_at
        FROM accounts
        WHERE address = $1 and block_number = $2
        "#,
    )
    .bind(address)
    .bind(block_number)
    .fetch_optional(pool)
    .await?;

    Ok(account)
}

pub async fn upsert_account_info(
    pool: &sqlx::PgPool,
    address: &str,
    balance: i64,
    block_number: i64,
) -> Result<Option<Account>, sqlx::Error> {
    let account = sqlx::query_as::<_, Account>(
        r#"
        INSERT INTO accounts (address, balance, block_number)
        VALUES ($1, $2, $3)
        ON CONFLICT (address) DO UPDATE
        SET balance = $2, block_number = $3
        RETURNING address, balance, block_number;
        "#,
    )
    .bind(address)
    .bind(balance)
    .bind(block_number)
    .fetch_optional(pool)
    .await?;

    Ok(account)
}
