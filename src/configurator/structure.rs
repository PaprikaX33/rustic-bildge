/*!
Defines the structure for the container
 */
use serde::{self, Deserialize, Serialize};
use std::path::PathBuf;

/// The main collector container
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthConfig {
    /// Location for the location where the file will be located
    #[serde(
        default = "AuthConfig::drop_def",
        deserialize_with = "crate::configurator::path_deserializer"
    )]
    pub drop_location: PathBuf,
    /// The IP binding for the server. Mostly for access control
    #[serde(default = "AuthConfig::bind_def")]
    pub bind: String,
    /// Port where the server binds
    #[serde(default = "AuthConfig::port_def")]
    pub port: u32,
    /// A flag to make the drop directory
    ///
    /// ## Note
    /// This feature is not implemented properly at the moment
    #[serde(default = "AuthConfig::gen_true")]
    pub generate_parent: bool,
    /// A flag to set whether rename file or reject it
    ///
    /// ## Note
    /// This feature is not implemented at all at the moment
    /// The behaviour is to rename
    //NOTE: This is not implemented yet
    #[serde(default = "AuthConfig::gen_false")]
    pub reject_collision: bool,
}
impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            drop_location: Self::drop_def(),
            bind: Self::bind_def(),
            port: Self::port_def(),
            generate_parent: Self::gen_true(),
            reject_collision: Self::gen_false(),
        }
    }
}
impl AuthConfig {
    fn drop_def() -> PathBuf {
        PathBuf::from("./dropbildge")
    }
    fn bind_def() -> String {
        "0.0.0.0".to_string()
    }
    fn port_def() -> u32 {
        8000
    }
    fn gen_true() -> bool {
        true
    }
    fn gen_false() -> bool {
        false
    }
}
