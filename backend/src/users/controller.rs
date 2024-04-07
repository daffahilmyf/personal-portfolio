use axum::{
    extract::Path, 
    http::StatusCode, 
    response::IntoResponse, 
    routing::get, 
    Extension, 
    Json, 
    Router
};
use serde_json::json;

use crate::utils::service::Services;

pub struct UserController;


impl UserController {
    pub fn app() -> Router {
        Router::new()
            .route("/:slug", get(Self::get_user_by_slug_endpoint))
    }

    pub async fn get_user_by_slug_endpoint(services: Extension<Services>, Path(slug): Path<String>) -> impl IntoResponse {
        let user = services.users.get_current_users(&slug).await;

        match user {
            Ok(user) => {
                match user {
                    Some(user) => (StatusCode::OK, Json(user)).into_response(),
                    None => (StatusCode::NOT_FOUND, Json(json!({
                        "error": "User not found"
                    }))).into_response()
                }
            },
            Err(error) => (StatusCode::NOT_FOUND, Json(json!({
                "error": error.to_string()
            }))).into_response()
        }
    }



}


