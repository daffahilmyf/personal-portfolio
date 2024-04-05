
use crate::utils::time::get_unix_timestamp;

use anyhow::Result;
use axum::async_trait;
use fake::Fake;
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::{FirstName, LastName};

use sqlx::prelude::FromRow;
use sqlx::types::Uuid;

use validator::Validate;

#[derive(Debug, FromRow, Validate)]
pub struct User {
    pub id: Uuid,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1))]
    pub first_name: String,

    #[validate(length(min = 1))]
    pub last_name: String,

    #[validate(length(min = 4, max = 4))]
    pub user_type: String,

    #[validate(length(min = 1))]
    pub slug: String,

    pub created_at: i64,
    pub updated_at: i64,
}

impl Default for User {
    fn default() -> Self {
        let first_name = FirstName().fake();
        let last_name = LastName().fake();
        let slug = format!("{}-{}", first_name, last_name).to_lowercase();

        Self {
            id: Uuid::new_v4(),
            email: SafeEmail().fake(),
            first_name: first_name,
            last_name: last_name,
            user_type: "1001".to_string(),
            slug: slug,
            created_at: get_unix_timestamp(),
            updated_at: get_unix_timestamp(),
        }
    }
}

#[async_trait]
pub trait UserRepository {
    async fn find_by_slug(&self, slug: &str) -> Result<Option<User>>;
    async fn get_users(&self) -> Result<Vec<User>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_default_user() {
        let user = User::default();
        assert!(user.first_name.len() > 0);
        assert!(user.last_name.len() > 0);
        assert!(user.email.len() > 0);
        assert!(user.slug.len() > 0);
        assert!(user.user_type.len() > 0);
        assert!(user.created_at > 0);
        assert!(user.updated_at > 0);
    }
}