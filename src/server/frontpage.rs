use axum::body::Body;
use axum::http::StatusCode;
use axum::response::Response;

pub async fn serve() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        //.header("X-Custom-Foo", "Bar")
        .body(Body::from(
            "<html><head><title>Rustic Bildge</title></head><body>
<div>Hello there</div>

<div><a href=\"/kill\">Kill</a> the server</div>

</body></html>",
        ))
        .unwrap()
}
