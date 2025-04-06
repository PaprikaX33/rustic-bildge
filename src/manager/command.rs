/*!
Command list for the messenging
 */
use std::path::PathBuf;
pub enum Command {
    Shutdown,
    NewDropLoc(PathBuf),
}
