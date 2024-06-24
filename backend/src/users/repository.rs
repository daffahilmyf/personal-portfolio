use std::sync::Arc;

use anyhow::{Context, Result};
use axum::async_trait;
use mockall::automock;
use crate::utils::database::Database;
use super::{model::User, query::{GET_USERS, GET_USER_BY_SLUG}};

pub type DynUsersRepository = Arc<dyn UserRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait UserRepository {
    async fn find_by_slug(&self, slug: &str) -> Result<Option<User>>;
    async fn get_users(&self) -> Result<Vec<User>>;
}


#[async_trait]
impl UserRepository for Database{
    async fn find_by_slug(&self, slug: &str) -> Result<Option<User>> {
        sqlx::query_as::<_, User>(GET_USER_BY_SLUG)
            .bind(slug)
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch user by slug")
    }

    async fn get_users(&self) -> Result<Vec<User>> {
        sqlx::query_as::<_, User>(GET_USERS)
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch users")
    }
}