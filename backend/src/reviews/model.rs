
use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    id: Uuid,
    reviewer_id: Uuid,
    reviewee_id: Uuid,
    description: String,
    review_type: String,
    created_at: i64,
    updated_at: i64,
}