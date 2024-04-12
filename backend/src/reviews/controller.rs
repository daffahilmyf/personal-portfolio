use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Extension, Router};

use crate::utils::{dto::ResponseData, service::Services, validate::validate_slug};

use super::model::ReviewWithUsers;

pub struct ReviewController;

impl ReviewController {
    pub fn app() -> Router {
        Router::new()
            .route("/:slug", get(Self::get_reviews_endpoint))
    }

    async fn get_reviews_endpoint(services: Extension<Services>, Path(slug): Path<String>) -> impl IntoResponse {

        if validate_slug(slug.as_str()) == false {
            let response = ResponseData::<()> {
                status: StatusCode::BAD_REQUEST.as_u16(),
                message: "Invalid Slug".to_string(),
                data: None
            };

            return (StatusCode::BAD_REQUEST, response.into_json()).into_response();
        }

        let reviews = services.reviews.get_reviews_by_user_slugs(&slug.as_str()).await;

        match reviews {
            Ok(reviews) => {
                let response = ResponseData::<Vec<ReviewWithUsers>> {
                    status: StatusCode::OK.as_u16(),
                    message: "Success".to_string(),
                    data: Some(reviews)
                };

                return (StatusCode::OK, response.into_json()).into_response();
            },
            Err(_) => {
                let response = ResponseData::<()> {
                    status: StatusCode::NOT_FOUND.as_u16(),
                    message: "Internal Server Error".to_string(),
                    data: None
                };

                return (StatusCode::NOT_FOUND, response.into_json()).into_response();
            }
            
        }

    }

}