use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Achievement {
    id: ObjectId,
    name: String,
    description: String,
    issuer: String,
    achievement_type: String,
}