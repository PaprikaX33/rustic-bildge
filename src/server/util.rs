use axum::{
    body::Body,
    response::Response,
    routing::{get, post},
};
pub async fn version() -> Response {
    Response::builder()
        .body(Body::from(env!("CARGO_PKG_VERSION")))
        .unwrap()
}
