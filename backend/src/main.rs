mod utils;
mod users;
mod routes;

use axum::{Extension, Router};
use dotenv::dotenv;
use utils::{ database::Database, service::Services};
use std::env::var;

use routes::app;

#[tokio::main]
async fn main(){

    dotenv().ok();

    let database_url = var("DATABASE_URI").unwrap();
    let db = Database::new(&database_url.to_string(), false).await.unwrap();

    let services = Services::new(db);

    let router = Router::new()
        .nest("/api/v1", app())
        .layer(Extension(services));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

