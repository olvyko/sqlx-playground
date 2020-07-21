mod services;

use services::*;
use std::sync::Arc;

pub struct UserModule {
    entry_usc: Arc<EntryUsecase>,
}

impl UserModule {
    pub fn new() -> UserModule {
        let entry_usc = Arc::new(EntryUsecase::new());
        UserModule { entry_usc }
    }

    pub fn entry(&self) -> Arc<EntryUsecase> {
        self.entry_usc.clone()
    }
}
