use crate::configurator::AuthConfig;
use crate::state::AppState;
mod backend;
mod frontpage;
use axum::{
    extract::{DefaultBodyLimit, State},
    response::Redirect,
    routing::{get, post},
    Router,
};
use tokio::runtime::Runtime;

/**
# Note
The wrapper for the tokio runtime.
All server function is done by [init_server]
**/
pub fn run_server(config: AuthConfig) -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Runtime::new()?;
    runtime.block_on(init_server(config));
    Ok(())
}

async fn init_server(config: AuthConfig) {
    let bind_form = format!("{}:{}", config.bind, config.port);
    let app_state = AppState::new(config);
    let data = match app_state.drop_location.clone().canonicalize() {
        Ok(val) => val.display().to_string(),
        Err(_) => "Invalid Path".to_string(),
    };
    let shutdown_trigger = app_state.shutdown_trigger.clone();
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
    let listener = tokio::net::TcpListener::bind(bind_form).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(async move { shutdown_trigger.notified().await })
        .await
        .expect("Failed to run server");
}

// Trigger shutdown handler
async fn shutdown(State(state): State<AppState>) -> &'static str {
    println!("Kill command received");
    state.kill();
    "Shutting down..."
}
