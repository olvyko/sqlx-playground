use crate::scalars::*;
pub use sqlx;
pub use sqlx::PgConnection;
use sqlx::{
    migrate::Migrator,
    postgres::{PgConnectOptions, PgPool, PgPoolOptions, PgSslMode},
};

pub type DbPool = PgPool;

pub struct DbPoolBuilder;
pub struct DbPoolFromOptions {
    options: PgConnectOptions,
}

pub struct DbPoolFromUrl<'a> {
    url: &'a str,
}

impl DbPoolBuilder {
    pub fn from_options(username: &str, password: &str, database: &str, host: &str, port: u16) -> DbPoolFromOptions {
        DbPoolFromOptions {
            options: PgConnectOptions::new()
                .host(host)
                .port(port)
                .database(database)
                .username(username)
                .password(password)
                .ssl_mode(PgSslMode::Disable),
        }
    }

    pub fn from_url(url: &str) -> DbPoolFromUrl {
        DbPoolFromUrl { url }
    }
}

impl DbPoolFromOptions {
    pub fn port(mut self, port: u16) -> Self {
        self.options = self.options.port(port);
        self
    }

    pub async fn build(self) -> Result<DbPool> {
        let pool = PgPoolOptions::new()
            .connect_timeout(std::time::Duration::from_secs(1))
            .connect_with(self.options)
            .await?;
        // Now its works only in runtime but soon `migrate!` macro will be avaliable
        let migrator = Migrator::new(std::path::Path::new("./migrations")).await?;
        migrator.run(&pool).await?;
        Ok(pool)
    }
}

impl<'a> DbPoolFromUrl<'a> {
    pub async fn build(self) -> Result<DbPool> {
        let pool = PgPoolOptions::new()
            .connect_timeout(std::time::Duration::from_secs(1))
            .connect(self.url)
            .await?;
        // Now its works only in runtime but soon `migrate!` macro will be avaliable
        let migrator = Migrator::new(std::path::Path::new("./migrations")).await?;
        migrator.run(&pool).await?;
        Ok(pool)
    }
}
