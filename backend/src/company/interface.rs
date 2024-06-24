
use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;


// crate::utils::EXPERIENCE_TYPE;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Company {
    id: ObjectId,
    name: String,
    description: String,
    created_at: Timestamp,
    updated_at: Timestamp,
}