/*!
Defines the structure for the container
 */
use serde::{self, Deserialize, Serialize};
use std::path::PathBuf;

/// The main collector container
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthConfig {
    /// Location for the location where the file will be located
    /// Default : [AuthConfig::drop_def]
    #[serde(
        default = "AuthConfig::drop_def",
        deserialize_with = "crate::configurator::path_deserializer"
    )]
    pub drop_location: PathBuf,
    /// The IP binding for the server. Mostly for access control
    ///
    /// Default : [AuthConfig::bind_def]
    #[serde(default = "AuthConfig::bind_def")]
    pub bind: String,
    /// Port where the server binds
    ///
    /// Default : [AuthConfig::port_def]
    #[serde(default = "AuthConfig::port_def")]
    pub port: u32,
    /// A flag to make the drop directory
    ///
    /// Default : [AuthConfig::gen_true]
    ///
    /// # Note
    /// This feature is not implemented properly at the moment
    #[serde(default = "AuthConfig::gen_true")]
    pub generate_parent: bool,
    /// A flag to set whether rename file or reject it
    ///
    /// Default : [AuthConfig::gen_false]
    ///
    /// # Note
    /// This feature is not implemented at all at the moment
    /// The behaviour is to rename
    ///
    /// # Warning
    /// Not implemented
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
    /// Default to `./dropbildge`
    fn drop_def() -> PathBuf {
        PathBuf::from("./dropbildge")
    }
    /// Default to `0.0.0.0`
    fn bind_def() -> String {
        "0.0.0.0".to_string()
    }
    /// Default to `8000`
    fn port_def() -> u32 {
        8000
    }
    /// return `true`
    fn gen_true() -> bool {
        true
    }
    /// return `false`
    fn gen_false() -> bool {
        false
    }
}
