use super::graphql::*;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GQLRequest, GQLResponse, WSSubscription};

pub async fn index(schema: web::Data<GraphqlSchema>, req: GQLRequest) -> GQLResponse {
    req.into_inner().execute(&schema).await.into()
}

pub async fn index_playground() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"))))
}

pub async fn index_ws(schema: web::Data<GraphqlSchema>, req: HttpRequest, payload: web::Payload) -> actix_web::Result<HttpResponse> {
    ws::start_with_protocols(WSSubscription::new(&schema), &["graphql-ws"], &req, payload)
}
