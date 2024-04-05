#![allow(dead_code)]

use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use tracing::info;

use super::{model::User, repository::DynUsersRepository};

pub type DynUsersService = Arc<dyn UserServiceTrait + Send + Sync>;


#[derive(Clone)]
pub struct UserService {
    pub repository: DynUsersRepository,
}

impl UserService {
    pub fn new(repository: DynUsersRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
pub trait UserServiceTrait {
    async fn get_current_users(&self, slug: &str) -> Result<Option<User>>;
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn get_current_users(&self, slug: &str) -> Result<Option<User>> {
        info!("Fetching user by slug: {}", slug);
        let user = self.repository.find_by_slug(slug).await?;

        Ok(user)
    }
}