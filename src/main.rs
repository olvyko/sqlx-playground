use shared::*;
use user::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let url = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL is not set in .env file")?;
    let pool = DbPoolBuilder::from_url(&url).build().await?;

    let user_module = UserModule::new();
    let entry = user_module.entry();

    {
        let mut tx = pool.begin().await?;
        entry.register(&mut tx, "admin", Some("admin@gmail.com")).await?;
    }
    {
        let mut tx = pool.begin().await?;
        entry.register(&mut tx, "admin", Some("admin@gmail.com")).await?;
        tx.commit().await?;
    }
    {
        let mut tx = pool.begin().await?;
        let error = entry.register(&mut tx, "admin", Some("admin@gmail.com")).await;
        println!("{:?}", error);
    }
    Ok(())
}
