use axum::body::Body;
use axum::http::{header::CONTENT_TYPE, HeaderName, HeaderValue, StatusCode};
use axum::response::Response;

static FAVICON_DATA: &'static [u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/ikon/icon.ico"));

pub async fn serve() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header(
            HeaderName::from_static("x-why-are-you-here"),
            HeaderValue::from_static("you are not supposed to see this"),
        )
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

pub async fn favicon() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(FAVICON_DATA))
        .unwrap()
}

pub async fn svgicon() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, HeaderValue::from_static("image/svg+xml"))
        .body(Body::from(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ikon/icon_min.svg"
        ))))
        .unwrap()
}
