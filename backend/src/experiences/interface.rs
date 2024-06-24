use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Experience {
    id: Uuid,
    name: String,
    description: String,
    issuer: String,
    // experience_type: EXPERIENCE_TYPE,
}