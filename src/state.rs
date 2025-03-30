/*!
Track the state of the program all the time
 */
use crate::configurator::AuthConfig;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Notify;

/**
Main State object
 **/

#[derive(Clone)]
pub struct AppState {
    pub shutdown_trigger: Arc<Notify>,
    pub drop_location: Arc<PathBuf>,
}

impl AppState {
    pub fn new(config: AuthConfig) -> AppState {
        AppState {
            shutdown_trigger: Arc::new(Notify::new()),
            drop_location: Arc::new(config.drop_location),
        }
    }
    pub fn kill(&self) {
        self.shutdown_trigger.notify_one();
    }
}
