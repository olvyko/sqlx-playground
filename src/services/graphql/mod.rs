mod user;

use self::user::*;

use crate::services::context::JuniperContext;
use juniper::{EmptySubscription, FieldResult, RootNode};
use shared::ServerScalarValue;

pub struct QueryRoot;

/// Query root
#[juniper::graphql_object(context = JuniperContext, scalar = ServerScalarValue)]
impl QueryRoot {
    /// User queries
    fn user() -> FieldResult<UserQueries, ServerScalarValue> {
        Ok(UserQueries)
    }
}

pub struct MutationRoot;

/// Mutation root
#[juniper::graphql_object(context = JuniperContext, scalar = ServerScalarValue)]
impl MutationRoot {
    /// User mutations
    fn user() -> FieldResult<UserMutations, ServerScalarValue> {
        Ok(UserMutations)
    }
}

/// Root query node of a schema.
/// This brings the mutation and query types together, and provides the predefined metadata fields.
pub type GraphqlRoot = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<JuniperContext>, ServerScalarValue>;

pub struct GraphqlRootBuilder;

impl GraphqlRootBuilder {
    pub fn build(self) -> GraphqlRoot {
        RootNode::new(QueryRoot, MutationRoot, EmptySubscription::new())
    }
}
