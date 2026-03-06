use std::env;

use sqlx::postgres::PgPoolOptions;

use crate::errors::ServerError;

pub async fn connect() -> Result<sqlx::PgPool, ServerError> {
    let database_url = env::var("DATABASE_URL").expect("Failed");
    let database_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed");

    Ok(database_pool)
}
