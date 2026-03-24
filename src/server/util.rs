use axum::{
    body::Body,
    response::Response,
    routing::{get, post},
};
#[cfg(feature = "util")]
pub async fn version() -> Response {
    Response::builder()
        .body(Body::from(env!("CARGO_PKG_VERSION")))
        .unwrap()
}
