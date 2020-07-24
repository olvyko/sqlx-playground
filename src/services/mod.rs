mod context;
mod graphql;
mod handlers;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use context::*;
use graphql::*;
use shared::*;
use std::sync::Arc;

pub async fn run() -> Result<()> {
    dotenv::dotenv().ok();
    let url = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL is not set in .env file")?;
    let db_pool = DbPoolBuilder::from_url(&url).build().await?;
    let modules = web::Data::new(ServerModules::new().await?);
    //let mut conn = db_pool.acquire().await?;
    // let c = modules
    //     .get_ref()
    //     .user()
    //     .entry()
    //     .get_customer_with_email_by_username(&mut conn, "admin")
    //     .await?;
    // println!("{:?}", c);
    let schema = Arc::new(GraphqlRootBuilder.build());
    let app = move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .wrap(Logger::default())
            .data(schema.clone())
            .data(db_pool.clone())
            .app_data(modules.clone())
            .service(handlers::graphql_handler)
            .service(handlers::playground_handler)
    };
    HttpServer::new(app).bind(format!("0.0.0.0:{}", 3000))?.run().await?;
    Ok(())
}
