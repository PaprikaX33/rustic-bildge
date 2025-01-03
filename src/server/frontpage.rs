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
