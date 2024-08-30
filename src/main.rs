use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{http::Response, Extension};
use axum::{middleware, Router};
use gp::authentication::auth;
use gp::establish_connection;
use gp::web::routes_signup::post_signup;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

async fn welcome() -> impl IntoResponse {
    "Welcome To Grace Pattiserie"
}

#[tokio::main]
async fn main() {
    let dbconn = establish_connection();
    let mdlw_db = dbconn.clone();

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let router = Router::new()
        .route("/", get(welcome))
        .route("/api/signup", post(post_signup))
        .layer(middleware::from_fn(move |req, next| {
            auth(mdlw_db.clone(), req, next)
        }))
        .layer(CookieManagerLayer::new())
        .layer(Extension(dbconn));

    axum::serve(listener, router).await.unwrap();
    Response::builder().status(http::StatusCode::ACCEPTED);
}
