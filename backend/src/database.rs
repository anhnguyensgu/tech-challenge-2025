use sqlx::{Pool, Postgres};

pub async fn init_pg_pool() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Pool::<Postgres>::connect(&database_url)
        .await
        .expect("failed to connect to database")
}
