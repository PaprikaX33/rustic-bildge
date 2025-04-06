/*!
The internal state of the axum's server.
Intended for all of the message passing mechanism within the axum
 */
use crate::manager;
use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, error::SendError},
    RwLock, RwLockReadGuard,
};

/// The Messenger object internal of the messenger object that perform the logic and everything
pub struct ServerManager {
    /// Drop location of the data
    ///
    /// ## Dev-Note
    /// RwLock is not cloneable, therefore wrapped in the Arc.
    /// Also there is no way to just take the reader in RwLock, hence taking the whole RwLock
    drop_location: Arc<RwLock<PathBuf>>,
    /// The sender object to send commands into the main state manager
    ///
    /// ## Dev-Note
    /// The sender is cloneable, therefore no need for Arc
    tx: mpsc::Sender<manager::Command>,
}

impl ServerManager {
    /// To get the drop location
    pub fn loc_getter(&self) -> impl Future<Output = RwLockReadGuard<'_, PathBuf>> {
        self.drop_location.read()
    }
    /// send the command to set new drop location
    pub fn new_loc(
        &self,
        path: PathBuf,
    ) -> impl Future<Output = Result<(), SendError<manager::Command>>> + use<'_> {
        self.tx.send(manager::Command::NewDropLoc(path))
    }
    /// Send the kill and shutdown command
    pub fn kill(&self) -> impl Future<Output = Result<(), SendError<manager::Command>>> + use<'_> {
        self.tx.send(manager::Command::Shutdown)
    }
}

impl From<manager::Manager> for ServerManager {
    fn from(man: manager::Manager) -> Self {
        Self {
            drop_location: man.get_drop(),
            tx: man.get_end_point(),
        }
    }
}
