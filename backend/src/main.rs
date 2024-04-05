use axum::{extract::Extension, response::IntoResponse, routing::get, serve, Json, Router};
use serde_json::json;
use sqlx::{PgPool, Row};
use tokio::net::TcpListener;



// Define the get_users function as before
async fn get_users(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let rows = match sqlx::query("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    };

    let users: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id": row.try_get::<i32, _>("id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
            })
        })
        .collect();

    (axum::http::StatusCode::OK, Json(users)).into_response()
}


#[tokio::main]
async fn main() {
    // Define Routes

    let database_url = "postgres://postgres:admin@localhost:5432/personal".to_string();
    let pool = PgPool::connect(&database_url).await.unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/users", get(get_users))
        .layer(Extension(pool));

    println!("Running on http://localhost:3000");
    // Start Server

    let tcp_listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    serve(tcp_listener, app.into_make_service())
        .await
        .unwrap();
}