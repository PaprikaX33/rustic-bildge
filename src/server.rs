use crate::configurator::AuthConfig;
use std::path::PathBuf;
mod backend;
mod frontpage;
use axum::{
    extract::{DefaultBodyLimit, State},
    response::Redirect,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Notify;

pub fn run_server(config: AuthConfig) -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Runtime::new()?;
    runtime.block_on(init_server(config));
    Ok(())
}

async fn init_server(config: AuthConfig) {
    let data = match config.drop_location.canonicalize() {
        Ok(val) => val.display().to_string(),
        Err(_) => "Invalid Path".to_string(),
    };
    let shutdown_trigger = Arc::new(Notify::new());
    let app_state = AppState {
        shutdown_trigger: shutdown_trigger.clone(),
        drop_location: Arc::new(config.drop_location),
    };

    // build our application with a single route
    let app = Router::new()
        .route("/debug", get(move || async move { format!("{:?}", data) }))
        .route("/kill", get(shutdown))
        .route("/index", get(frontpage::serve))
        .route("/", get(|| async { Redirect::permanent("/index") }))
        .route("/receiver", post(backend::receive))
        .layer(DefaultBodyLimit::disable())
        .fallback(frontpage::invalid)
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
    drop_location: Arc<PathBuf>,
}

// Trigger shutdown handler
async fn shutdown(State(state): State<AppState>) -> &'static str {
    println!("Kill command received");
    state.shutdown_trigger.notify_one();
    "Shutting down..."
}
