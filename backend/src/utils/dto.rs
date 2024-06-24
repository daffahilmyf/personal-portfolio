use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData <T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>
}

impl <T> ResponseData<T> {
    pub fn into_json(self) -> Json<Self> {
        Json(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_json() {
        let response = ResponseData::<()> {
            status: 200,
            message: "Success".to_string(),
            data: None
        };

        let json = response.into_json();

        assert_eq!(json.status, 200);
        assert_eq!(json.message, "Success".to_string());
    }
}