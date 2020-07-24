use super::{
    context::{JuniperContext, ServerModules},
    graphql::GraphqlRoot,
};
use actix_web::{get, post, web, Error, HttpResponse, Responder};
use juniper::http::{playground::playground_source, GraphQLRequest};
use shared::{DbPool, ServerScalarValue};
use std::sync::Arc;

#[get("/playground")]
pub async fn playground_handler() -> impl Responder {
    let html = playground_source("/graphql", None);
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
}

#[post("/graphql")]
pub async fn graphql_handler(
    graphql_root: web::Data<Arc<GraphqlRoot>>,
    req: web::Json<GraphQLRequest<ServerScalarValue>>,
    db_pool: web::Data<DbPool>,
    modules: web::Data<ServerModules>,
) -> Result<impl Responder, Error> {
    let ctx = JuniperContext::init(db_pool.get_ref().clone(), modules.into_inner());
    let res = req.execute(&graphql_root, &ctx).await;
    let json = serde_json::to_string(&res)?;
    Ok(HttpResponse::Ok().content_type("application/json").body(json))
}
