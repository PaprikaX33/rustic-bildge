mod configurator;
use axum::{extract::State, routing::get, Router};
use std::sync::Arc;
use tokio::sync::Notify;

#[tokio::main]
async fn main() {
    let config = configurator::load_config();
    let data = match config.drop_location.canonicalize() {
        Ok(val) => val.display().to_string(),
        Err(_) => "Invalid Path".to_string(),
    };
    let shutdown_trigger = Arc::new(Notify::new());
    let app_state = AppState {
        shutdown_trigger: shutdown_trigger.clone(),
    };

    // build our application with a single route
    let app = Router::new()
        .route("/", get(move || async move { format!("{:?}", data) }))
        .route("/kill", get(shutdown))
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.bind, config.port))
        .await
        .unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(async move { shutdown_trigger.notified().await })
        .await
        .expect("Failed to run server");
}

#[derive(Clone)]
struct AppState {
    shutdown_trigger: Arc<Notify>,
}

// Trigger shutdown handler
async fn shutdown(State(state): State<AppState>) -> &'static str {
    println!("Kill command received");
    state.shutdown_trigger.notify_one();
    "Shutting down..."
}
