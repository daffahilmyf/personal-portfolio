use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;


// crate::utils::EXPERIENCE_TYPE;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Experience {
    id: ObjectId,
    name: String,
    description: String,
    issuer: String,
    // experience_type: EXPERIENCE_TYPE,
}