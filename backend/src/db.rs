use sqlx::postgres::PgPoolOptions;

pub async fn init_db() -> Result<sqlx::PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = PgPoolOptions::new().connect(&database_url).await?;
    Ok(pool)
}
