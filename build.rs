use sqlx::{migrate::Migrator, postgres::PgPoolOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let url = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL is not set in .env file")?;
    if let Ok(pool) = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(1))
        .connect(&url)
        .await
    {
        // Now its works only in runtime but soon `migrate!` macro will be avaliable
        let migrator = Migrator::new(std::path::Path::new("./migrations")).await?;
        migrator.run(&pool).await?;
    }
    Ok(())
}
