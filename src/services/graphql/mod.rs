use async_graphql::{Context, Schema};
use futures::{Stream, StreamExt};
use std::time::Duration;

pub type GraphqlSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn register(&self, ctx: &Context<'_>) -> String {
        "id".to_owned()
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn register(&self, ctx: &Context<'_>, username: String, email: Option<String>) -> String {
        "id".to_owned()
    }
}

pub struct SubscriptionRoot;

#[async_graphql::Subscription]
impl SubscriptionRoot {
    async fn interval(&self, #[arg(default = 1)] n: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        tokio::time::interval(Duration::from_secs(1)).map(move |_| {
            value += n;
            value
        })
    }
}
