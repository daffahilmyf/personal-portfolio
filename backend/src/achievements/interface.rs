use serde::{Serialize, Deserialize};

use sqlx::types::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Achievement {
    id: Uuid,
    name: String,
    description: String,
    issuer: String,
    achievement_type: String,
}