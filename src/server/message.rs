/*!
The internal state of the axum's server.
Intended for all of the message passing mechanism within the axum
 */
use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, error::SendError},
    RwLock, RwLockReadGuard,
};

/// Main Messenger that wrap the inner messenger into an Arc to make it cloneable
#[derive(Clone)]
pub struct Messenger(Arc<MessengerInner>);

/// The internal of the messenger object that perform the logic and everything
pub struct MessengerInner {
    /// Drop location
    drop_location: RwLock<String>,
    /// The sender object to send commands into the main state manager
    tx: mpsc::Sender<Command>,
}

/// Deref trait for the Messenger object to make the operation transparent
impl std::ops::Deref for Messenger {
    type Target = MessengerInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MessengerInner {
    /// To get the drop location
    pub fn loc_getter(&self) -> impl Future<Output = RwLockReadGuard<'_, String>> {
        self.drop_location.read()
    }
    /// send the command to set new drop location
    pub fn new_loc(
        &self,
        path: PathBuf,
    ) -> impl Future<Output = Result<(), SendError<Command>>> + use<'_> {
        self.tx.send(Command::NewDropLoc(path))
    }
    /// Send the kill and shutdown command
    pub fn kill(&self) -> impl Future<Output = Result<(), SendError<Command>>> + use<'_> {
        self.tx.send(Command::Shutdown)
    }
}

pub enum Command {
    Shutdown,
    NewDropLoc(PathBuf),
}
