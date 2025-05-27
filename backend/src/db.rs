use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing");
    PgPoolOptions::new().connect(&database_url).await
}
