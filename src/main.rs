use axum::http::Response;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use sqlx::Database;
use tokio::net::TcpListener;

async fn welcome() -> impl IntoResponse {
    "Welcome To Grace Pattiserie"
}

#[tokio::main]
async fn main() {
    
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let router = Router::new().route("/", get(welcome));

    axum::serve(listener, router).await.unwrap();
    Response::builder().status(http::StatusCode::ACCEPTED);
}
