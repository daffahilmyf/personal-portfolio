use std::sync::Arc;

use anyhow::Error;
use axum::async_trait;
use tracing::info;

use super::{model:: ReviewWithUsers, repository::DynReviewsRepository};

pub type DynReviewsService = Arc<dyn ReviewServiceTrait + Send + Sync>;

#[derive(Clone)]
pub struct ReviewService {
    pub repository: DynReviewsRepository,
}

impl ReviewService {
    pub fn new(repository: DynReviewsRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
pub trait ReviewServiceTrait {
    async fn get_reviews_by_user_slugs(&self, slug: &str) -> Result<Vec<ReviewWithUsers>, Error>;
}

#[async_trait]
impl ReviewServiceTrait for ReviewService {
    async fn get_reviews_by_user_slugs(&self, slug: &str) -> Result<Vec<ReviewWithUsers>, Error> {
        info!("Fetching review by slug: {}", slug);

        self.repository.find_by_user_slug(slug).await
    }
}