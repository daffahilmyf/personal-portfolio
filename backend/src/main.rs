use axum::{routing::get, Router};


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URI").expect("DATABASE_URI must be set");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
