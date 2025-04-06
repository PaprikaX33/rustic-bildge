/*!
Manager of the whole program. Responsible for all action and the main state of the program
 */
mod command;
pub use command::Command;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, error::SendError},
    RwLock, RwLockReadGuard,
};
pub struct Manager {
    drop_location: Arc<RwLock<PathBuf>>,
    cmd: CommandChannelPair,
}

/// Bundling the command for easy
struct CommandChannelPair {
    pub tx: mpsc::Sender<Command>,
    pub rx: mpsc::Receiver<Command>,
}

impl Manager {
    pub fn get_drop(&self) -> Arc<RwLock<PathBuf>> {
        self.drop_location.clone()
    }
    pub fn get_end_point(&self) -> mpsc::Sender<Command> {
        self.cmd.tx.clone()
    }
}
