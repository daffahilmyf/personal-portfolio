mod utils;
mod users;

use anyhow::Context;
use axum::{routing::get, Extension, Router};
use dotenv::dotenv;
use users::controller::UserController;
use utils::{ database::Database, service};
use std::env::var;

pub async fn health() -> &'static str {
    "🚀🚀🚀 Server Running"
}

pub fn app() -> Router {
    Router::new()
        .nest("/users", UserController::app())
        .route("/health", get(health))
}

#[tokio::main]
async fn main(){

    dotenv().ok();

    let database_url = var("DATABASE_URI").unwrap();
    let db = Database::new(&database_url.to_string(), false).await.unwrap();

    let services = service::Services::new(db);

    let router = Router::new()
        .nest("/api/v1", app())
        .route("/", get(health))
        .layer(Extension(services));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .context("Failed to bind server");
}

