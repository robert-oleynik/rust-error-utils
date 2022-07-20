use std::path::Path;

use error_utils::Errors;

#[derive(Debug, Errors)]
enum ConfigError {
	#[error("Failed to read config file (Reason: {})", from)]
	Io(std::io::Error),
	#[error("Failed to parse config file (Reason: {})", from)]
	Toml(toml::de::Error),
}

fn read_config<P: AsRef<Path>>(path: P) -> Result<toml::Value, ConfigError> {
	let content = std::fs::read_to_string(path)?;
	Ok(toml::from_str(&content)?)
}

fn main() -> Result<(), ConfigError> {
	let _config = read_config("Cargo.toml")?;
	Ok(())
}
