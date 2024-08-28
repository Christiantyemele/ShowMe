use axum::{http::Response, Extension};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use gp::establish_connection;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;


async fn welcome() -> impl IntoResponse {
    "Welcome To Grace Pattiserie"
}

#[tokio::main]
async fn main() {
    let dbconn = establish_connection();
    
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let router = Router::new()
    .route("/", get(welcome))
    .layer(CookieManagerLayer::new())
    .layer(Extension(dbconn));

    axum::serve(listener, router).await.unwrap();
    Response::builder().status(http::StatusCode::ACCEPTED);
}
