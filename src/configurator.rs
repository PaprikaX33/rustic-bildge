use serde::{self, Deserialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use toml;

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    #[serde(
        default = "AuthConfig::default_drop_dir",
        deserialize_with = "path_deserializer"
    )]
    drop_location: PathBuf,
    #[serde(default = "AuthConfig::default_port")]
    port: u32,
}
impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            drop_location: Self::default_drop_dir(),
            port: Self::default_port(),
        }
    }
}
impl AuthConfig {
    fn default_drop_dir() -> PathBuf {
        PathBuf::from("./dropbildge")
    }

    fn default_port() -> u32 {
        8000
    }
}
pub fn load_config() -> AuthConfig {
    // Determine the package root for debug mode
    let package_root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let default_config_path = if cfg!(debug_assertions) {
        // Debug mode: use the path relative to the package root
        Path::new(&package_root).join("config.toml")
    } else {
        // Release mode: locate right next to the executable
        PathBuf::from("./config.toml")
    };

    // Check for `-c` argument for override of config path
    let config_path = env::args()
        .enumerate()
        .find(|(_i, arg)| arg == "-c")
        .and_then(|(i, _)| env::args().nth(i + 1))
        .map(PathBuf::from)
        .unwrap_or(default_config_path);

    if !config_path.exists() {
        return AuthConfig {
            ..Default::default()
        };
    }
    // Print out the path for demonstration purposes
    println!("Configuration used: {:?}", config_path);

    let config_content =
        fs::read_to_string(config_path).expect("Failed to read configuration file");

    // Parse the TOML
    let config: AuthConfig =
        toml::from_str(&config_content).expect("Failed to parse configuration file");
    println! {"Config loaded successfully"};
    return config;
}

fn path_deserializer<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(PathBuf::from(s))
}
