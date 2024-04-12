
use std::sync::Arc;
use tracing::info;


use crate::users::service::{DynUsersService, UserService};

use super::database::Database;

#[derive(Clone)]
pub struct Services {
    pub users: DynUsersService,
}

impl Services {
    pub fn new(db: Database) -> Self {
        info!("initializing utility services...");
        let repository = Arc::new(db);
        let users = Arc::new(UserService::new(repository.clone())) as DynUsersService;

        Self { users }
    }
}
