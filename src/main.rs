mod configurator;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let config = configurator::load_config();
    let data = match config.drop_location.canonicalize() {
        Ok(val) => val.display().to_string(),
        Err(_) => "Invalid Path".to_string(),
    };
    // build our application with a single route
    let app = Router::new().route(
        "/",
        get(move || async move { format!("Hello, World! drop on {}", data) }),
    );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
