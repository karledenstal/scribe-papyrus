use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::config;

/// Initialize a new scribe project by creating scribe.toml
pub fn run() -> Result<()> {
    let config_path = Path::new("scribe.toml");

    // Check if scribe.toml already exists
    if config_path.exists() {
        anyhow::bail!("scribe.toml already exists in current directory");
    }

    // Write the default template to scribe.toml
    fs::write(config_path, config::default_config_template())?;

    println!("✓ Created scribe.toml in current directory");

    Ok(())
}
