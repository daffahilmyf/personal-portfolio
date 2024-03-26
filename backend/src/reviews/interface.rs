
use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    id: ObjectId,
    user_id: ObjectId,
    
}