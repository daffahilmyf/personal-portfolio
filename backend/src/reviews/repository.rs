use std::sync::Arc;

use anyhow::{Context, Error};
use axum::async_trait;
use mockall::automock;

use crate::utils::database::Database;

use super::{model::ReviewWithUsers, query::GET_USER_REVIEW_BY_SLUG};

pub type DynReviewsRepository = Arc<dyn ReviewRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait ReviewRepository {
    async fn find_by_user_slug(&self, slug: &str) -> Result<Vec<ReviewWithUsers>, Error>;}

#[async_trait]
impl ReviewRepository for Database {
    async fn find_by_user_slug(&self, slug: &str) -> Result<Vec<ReviewWithUsers>, Error> {
        sqlx::query_as::<_, ReviewWithUsers>(GET_USER_REVIEW_BY_SLUG)
            .bind(slug)
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch users")
    }
}