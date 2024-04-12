
use crate::utils::time::get_unix_timestamp;

use serde::{Serialize, Deserialize};
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(Debug, Serialize, FromRow, Deserialize)]
pub struct Review {
    id: Uuid,
    reviewer_id: Uuid,
    reviewee_id: Uuid,
    description: String,
    review_type: String,
    created_at: i64,
    updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ReviewWithUsers {
    id: Uuid,
    reviewer_id: Uuid,
    reviewee_id: Uuid,
    reviewer_first_name: String,
    reviewer_last_name: String,
    reviewee_first_name: String,
    reviewee_last_name: String,
    description: String,
    review_type: String,
    created_at: i64,
    updated_at: i64,
}

impl Default for Review {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            reviewer_id: Uuid::new_v4(),
            reviewee_id: Uuid::new_v4(),
            description: "".to_string(),
            review_type: "".to_string(),
            created_at: get_unix_timestamp(),
            updated_at: get_unix_timestamp(),
        }
    }
}

