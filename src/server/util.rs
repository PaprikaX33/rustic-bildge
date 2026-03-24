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

#[cfg(feature = "util")]
pub async fn features() -> Response {
    Response::builder()
        .body(Body::from(format!(
            "{} {}",
            if cfg!(feature = "util") { "util" } else { "" },
            if cfg!(feature = "tray") { "tray" } else { "" },
        )))
        .unwrap()
}
