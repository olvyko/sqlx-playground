mod graphql;
mod handlers;

use actix_cors::Cors;
use actix_web::{guard, http::header, middleware::Logger, web, App, HttpServer};
use async_graphql::Schema;
use graphql::*;
use handlers::*;
use shared::*;

pub async fn run() -> Result<()> {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot).finish();
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
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    };
    HttpServer::new(app).bind(format!("0.0.0.0:{}", 8080))?.run().await?;
    Ok(())
}
