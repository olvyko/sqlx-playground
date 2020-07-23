mod user;

use self::user::*;

use crate::services::context::JuniperContext;
use juniper::{EmptySubscription, FieldResult, RootNode};

pub struct QueryRoot;

/// Query root
#[juniper::graphql_object(context = JuniperContext)]
impl QueryRoot {
    /// User queries
    fn user() -> FieldResult<UserQueries> {
        Ok(UserQueries)
    }
}

pub struct MutationRoot;

/// Mutation root
#[juniper::graphql_object(context = JuniperContext)]
impl MutationRoot {
    /// User mutations
    fn user() -> FieldResult<UserMutations> {
        Ok(UserMutations)
    }
}

/// Root query node of a schema.
/// This brings the mutation and query types together, and provides the predefined metadata fields.
pub type GraphqlRoot = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<JuniperContext>>;

pub struct GraphqlRootBuilder;

impl GraphqlRootBuilder {
    pub fn build(self) -> GraphqlRoot {
        RootNode::new(QueryRoot, MutationRoot, EmptySubscription::new())
    }
}
