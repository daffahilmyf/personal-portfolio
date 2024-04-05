
use axum::http::Extensions;
use axum::Extension;
use axum::{extract::Path, Router, debug_handler};
use axum::routing::get;


use crate::utils::error::AppResult;
use crate::utils::service::Services;
use super::model::User;

pub struct UserController;


// impl UserController {
//     #[debug_handler]
//     pub fn app() -> Router {
//         Router::new()
//             .route("/:slug", get(get_current_users_endpoint))
//     }



// }


pub async fn get_current_users_endpoint(services: Extension<Services>, Path(slug): Path<&str>) -> AppResult<Option<User>> {
    let user = services.users.get_current_users(slug).await?;
    Ok(user)
}