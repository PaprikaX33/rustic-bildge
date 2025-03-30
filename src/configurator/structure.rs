use serde::{self, Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthConfig {
    #[serde(
        default = "AuthConfig::drop_def",
        deserialize_with = "crate::configurator::path_deserializer"
    )]
    pub drop_location: PathBuf,
    #[serde(default = "AuthConfig::bind_def")]
    pub bind: String,
    #[serde(default = "AuthConfig::port_def")]
    pub port: u32,
    #[serde(default = "AuthConfig::gen_true")]
    pub generate_parent: bool,
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
