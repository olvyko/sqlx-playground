mod services;

use colored::Colorize;
use log::LevelFilter;
use shared::Result;
use std::future::Future;

#[actix_rt::main]
async fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .filter(Some("actix_web"), LevelFilter::Error)
        .init();
    handle_errors(services::run).await;
}

async fn handle_errors<F>(run: impl FnOnce() -> F)
where
    F: Future<Output = Result<()>>,
{
    if let Err(err) = run().await {
        eprintln!("{}: {}", "error".red().bold(), err);
        if let Some(source) = err.source() {
            eprintln!("{}: {}", "caused by".bright_red().bold(), source);
        }
        std::process::exit(1);
    };
}
