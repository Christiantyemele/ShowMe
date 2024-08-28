use axum::response::IntoResponse;
use cookie::Cookie;
use tower_cookies::Cookies;

use crate::{SessionToken, AUTH_COOKIE_NAME};

pub fn login_response(cookie: Cookies, session_tk: SessionToken) -> impl IntoResponse {
    cookie.add(Cookie::new(
        AUTH_COOKIE_NAME,
        session_tk.into_cookie_value(),
    ))
}
