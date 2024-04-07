use axum::{
    http::StatusCode,
    response::IntoResponse,
    Router,
    routing::get
};
use crate::users::controller::UserController;
    
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "🚀🚀🚀 Server Running").into_response()
}

pub fn app() -> Router {
    Router::new()
        .route("/", get(health))
        .nest("/users", UserController::app())
}
