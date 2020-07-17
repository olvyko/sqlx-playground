mod services;

use services::*;
use std::sync::Arc;

pub struct UserModule {
    entry_ctr: Arc<EntryController>,
}

impl UserModule {
    pub fn new() -> UserModule {
        let entry_ctr = Arc::new(EntryController::new());

        UserModule { entry_ctr }
    }

    pub fn entry(&self) -> Arc<EntryController> {
        self.entry_ctr.clone()
    }
}
