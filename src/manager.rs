/*!
Global Manager of the whole program.
 */
mod command;
use crate::configurator::AuthConfig;
pub use command::Command;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, error::SendError},
    Notify, RwLock, RwLockReadGuard,
};
pub struct Manager {
    drop_location: Arc<RwLock<PathBuf>>,
    port: Arc<RwLock<u32>>,
    cmd: CommandChannelPair<Command>,
    shutdown_trigger: Arc<Notify>,
}

/// Channel bundling to make it easy to manage
struct CommandChannelPair<T> {
    pub tx: mpsc::Sender<T>,
    pub rx: mpsc::Receiver<T>,
}
impl<T> From<(mpsc::Sender<T>, mpsc::Receiver<T>)> for CommandChannelPair<T> {
    fn from(pair: (mpsc::Sender<T>, mpsc::Receiver<T>)) -> Self {
        Self {
            tx: pair.0,
            rx: pair.1,
        }
    }
}

impl Manager {
    pub fn get_drop(&self) -> Arc<RwLock<PathBuf>> {
        self.drop_location.clone()
    }
    pub fn get_end_point(&self) -> mpsc::Sender<Command> {
        self.cmd.tx.clone()
    }

    pub fn new(config: AuthConfig) -> Self {
        Self {
            drop_location: Arc::new(RwLock::new(config.drop_location)),
            shutdown_trigger: Arc::new(Notify::new()),
            port: Arc::new(RwLock::new(config.port)),
            cmd: mpsc::channel(10).into(),
        }
    }
}
