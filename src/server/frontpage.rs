use axum::body::Body;
use axum::http::StatusCode;
use axum::response::Response;

pub async fn serve() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/html/index.html"
        ))))
        .unwrap()
}

pub async fn invalid() -> Response {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/html/404.html"
        ))))
        .unwrap()
}
