use serde::{Deserialize, Deserializer};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
pub use structure::AuthConfig;
mod structure;

pub fn load_config(directpath: Option<PathBuf>) -> AuthConfig {
    let config_path = directpath.unwrap_or({
        if cfg!(debug_assertions) {
            // Debug mode: use the path relative to the package root
            // Get the package root
            let package_root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
            Path::new(&package_root).join("config.toml")
        } else {
            // Release mode: locate right next to the executable
            PathBuf::from("./config.toml")
        }
    });
    if !config_path.exists() {
        return AuthConfig::default();
    }
    // Print out the path for demonstration purposes
    println!("Configuration used: {:?}", config_path);

    let config_content =
        fs::read_to_string(config_path).expect("Failed to read configuration file");

    // Parse the TOML
    let config: AuthConfig =
        toml::from_str(&config_content).expect("Failed to parse configuration file");
    println! {"Config loaded successfully"};
    config
}

pub fn generate_boilerplate_config(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let config = AuthConfig::default();
    fs::write(path, toml::to_string_pretty(&config)?)?;
    Ok(())
}

fn path_deserializer<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(PathBuf::from(s))
}
