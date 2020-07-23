use shared::*;
use std::sync::Arc;
use user::*;

#[derive(Clone)]
pub struct ServerModules {
    user: Arc<UserModule>,
}

impl ServerModules {
    pub async fn new() -> Result<Self> {
        let user = Arc::new(UserModule::new());
        Ok(ServerModules { user })
    }

    pub fn user(&self) -> Arc<UserModule> {
        self.user.clone()
    }
}

#[derive(Clone)]
pub struct JuniperContext {
    db_pool: DbPool,
    modules: Arc<ServerModules>,
}

impl JuniperContext {
    pub fn init(db_pool: DbPool, modules: Arc<ServerModules>) -> Self {
        JuniperContext { db_pool, modules }
    }

    pub fn db_pool(&self) -> DbPool {
        self.db_pool.clone()
    }

    pub fn modules(&self) -> Arc<ServerModules> {
        self.modules.clone()
    }
}

impl juniper::Context for JuniperContext {}
