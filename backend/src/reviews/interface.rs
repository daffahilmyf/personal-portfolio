
use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    id: Uuid,
    user_id: Uuid,
    
}