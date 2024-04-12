use axum::{
    extract::Path, 
    http::StatusCode, 
    response::IntoResponse, 
    routing::get, 
    Extension, 
    Router
};

use crate::utils::{dto::ResponseData, service::Services};

use super::model::User;

pub struct UserController;


impl UserController {
    pub fn app() -> Router {
        Router::new()
            .route("/:slug", get(Self::get_user_by_slug_endpoint))
            .route("/", get(Self::get_users_endpoint))
    }

    pub async fn get_user_by_slug_endpoint(services: Extension<Services>, Path(slug): Path<String>) -> impl IntoResponse {
        let user = services.users.get_current_users(&slug).await.unwrap();

  
        match user {
            Some(user) => {
                let response = ResponseData::<User> {
                    status: StatusCode::OK.as_u16(),
                    message: "Success".to_string(),
                    data: Some(user)
                };

                return (StatusCode::OK, response.into_json()).into_response();
            },
            None => {
                let response = ResponseData::<()> {
                    status: StatusCode::NOT_FOUND.as_u16(),
                    message: "User not found".to_string(),
                    data: None
                };

                return (StatusCode::NOT_FOUND, response.into_json()).into_response();
            }
        }
    }

    async fn get_users_endpoint(services: Extension<Services>) -> impl IntoResponse {
        let users = services.users.get_users().await.unwrap();

        let response = ResponseData::<Vec<User>> {
            status: StatusCode::OK.as_u16(),
            message: "Success".to_string(),
            data: Some(users)
        };

        (StatusCode::OK, response.into_json()).into_response()
    }
}


