mod shared;
mod user;

use async_trait::async_trait;
use shared::*;
use sqlx::migrate::Migrator;
use sqlx::{PgConnection, PgPool};
use user::*;

// 1. Every usecase is trait, `PgConnection` should derive usecase trait
// 2. Every controller is struct
// 3. Every controller's method which works with a database should have `&mut PgConnection` argument

#[async_trait]
pub trait Truncate {
    async fn truncate_all_tables(&mut self) -> Result<()>;
}

#[async_trait]
impl Truncate for PgConnection {
    async fn truncate_all_tables(&mut self) -> Result<()> {
        sqlx::query!("TRUNCATE TABLE customer, email").execute(self).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let url = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL is not set in .env file")?;
    let pool = PgPool::connect(&url).await?;

    // Now its works only in runtime but soon `migrate!` macro will be avaliable
    let migrator = Migrator::new(std::path::Path::new("./migrations")).await?;
    migrator.run(&pool).await?;

    let user_module = UserModule::new();
    let entry_ctr = user_module.entry();

    let mut conn = pool.acquire().await?;
    conn.truncate_all_tables().await?;
    {
        let mut tx = pool.begin().await?;
        entry_ctr.register(&mut tx, "admin", Some("admin@gmail.com")).await?;
    }
    {
        let mut tx = pool.begin().await?;
        entry_ctr.register(&mut tx, "admin", Some("admin@gmail.com")).await?;
        tx.commit().await?;
    }
    {
        let mut tx = pool.begin().await?;
        let error = entry_ctr.register(&mut tx, "admin", Some("admin@gmail.com")).await;
        println!("{:?}", error);
    }
    Ok(())
}
