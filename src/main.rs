use std::sync::{Arc, Mutex};

use axum::response::IntoResponse;
use axum::routing::{get, post, post_service};
use axum::{http::Response, Extension};
use axum::{middleware, Router};
use gp::authentication::auth;

use gp::establish_connection;
use gp::utils::logout_response;
use gp::web::routes_signup::{post_delete_me, post_login, post_signup};
use rand_chacha::ChaCha8Rng;
use rand_core::{OsRng, RngCore, SeedableRng};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

async fn welcome() -> impl IntoResponse {
    "Welcome To Grace Pattiserie"
}

#[tokio::main]
async fn main() {
    let dbconn: deadpool_diesel::Pool<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    > = establish_connection().await;

    let mdlw_db = dbconn.clone();
    let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let router = Router::new()
        .route("/", get(welcome))
        .route("/signup", post(post_signup))
        .route("/login", post(post_login))
        .route("/logout", post(logout_response))
        .route("/delete", post(post_delete_me))
        .layer(middleware::from_fn(move |req, next| {
            auth(mdlw_db.clone(), req, next)
        }))
        .layer(CookieManagerLayer::new())
        .layer(Extension(dbconn))
        .layer(Extension(Arc::new(Mutex::new(random))));

    axum::serve(listener, router).await.unwrap();
    Response::builder().status(http::StatusCode::ACCEPTED);
}
#[cfg(test)]
#[tokio::test]
async fn api_tes() {
    use gp::authentication::SignupPayload;
    let payload = SignupPayload {
        username: "christian".to_owned(),
        password:
            "sdafiuakffdsaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaajpertre"
                .to_owned(),
    };
    let req = reqwest::Client::new();
    req.post("http://localhost:8080/api/signup")
        .body(serde_json::to_string(&payload).unwrap())
        .send()
        .await
        .unwrap();
}
