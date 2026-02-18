use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::bundled;

#[derive(Debug, Deserialize, Serialize)]
pub struct ScribeConfig {
    pub name: String,
    pub mods: PathBuf,
    pub game: String,
    pub scripts: PathBuf,
    pub compiler: PathBuf,
    pub dependencies: HashMap<String, String>,
    #[serde(rename = "sourceDir")]
    pub source_dir: Option<PathBuf>,
    #[serde(rename = "outputDir")]
    pub output_dir: Option<PathBuf>,
}

/// Returns the default scribe.toml template string
pub fn default_config_template() -> String {
    let compiler_path = bundled::default_compiler_path();

    format!(
        r#"# Scribe Papyrus Project Configuration
# Place this file in your workspace root directory

name = "My Awesome Mod"
mods = "/path/to/mods/folder"
game = "sse"
scripts = "/path/to/Data/Scripts/Source"

# Path to the papyrus compiler executable
# Default: bundled compiler at ~/.scribe/bin/papyrus
# Can also be an absolute path or use placeholders
compiler = "{}"

# Define your dependencies here
# Use {{mods}}, {{scripts}}, {{game}}, or {{name}} as placeholders
[dependencies]
skse = "{{mods}}/skse/scripts/source"
skyui = "{{mods}}/skyui/scripts/source"
papyrusutil = "{{mods}}/PapyrusUtil/scripts/source"

# Optional: customize source and output directories (relative to workspace root)
sourceDir = "./src/scripts/source"
outputDir = "./src/scripts"
"#,
        compiler_path
    )
}

/// Find the scribe.toml config file by searching upward from the current directory
pub fn find_config(start_dir: &Path, custom: Option<PathBuf>) -> Result<PathBuf> {
    // If custom path provided, use it directly
    if let Some(custom_path) = custom {
        if !custom_path.exists() {
            anyhow::bail!(
                "Custom config file '{}' does not exist",
                custom_path.display()
            );
        }
        return Ok(custom_path);
    }

    // Search upward from start_dir for scribe.toml
    let mut current = start_dir.to_path_buf();
    loop {
        let config_path = current.join("scribe.toml");
        if config_path.exists() {
            return Ok(config_path);
        }

        // Try to go to parent directory
        if !current.pop() {
            // Reached filesystem root without finding config
            anyhow::bail!(
                "Could not find scribe.toml in current directory or any parent directory.\n\
                 Run 'scribe init' to create one."
            );
        }
    }
}

/// Load and parse the scribe.toml config file
pub fn load_config(config_path: &Path) -> Result<ScribeConfig> {
    let contents = fs::read_to_string(config_path).context(format!(
        "Failed to read config file: {}",
        config_path.display()
    ))?;

    let config: ScribeConfig = toml::from_str(&contents).context(format!(
        "Failed to parse config file: {}",
        config_path.display()
    ))?;

    Ok(config)
}

/// Resolve placeholders in a path string
/// Replaces {mods}, {scripts}, {game}, and {name} with values from config
pub fn resolve_placeholders(path: &str, config: &ScribeConfig) -> String {
    path.replace("{mods}", &config.mods.to_string_lossy())
        .replace("{scripts}", &config.scripts.to_string_lossy())
        .replace("{game}", &config.game)
        .replace("{name}", &config.name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_placeholders() {
        let config = ScribeConfig {
            name: "TestMod".to_string(),
            mods: PathBuf::from("/mods"),
            game: "sse".to_string(),
            scripts: PathBuf::from("/scripts"),
            compiler: PathBuf::from("/compiler/papyrus"),
            dependencies: HashMap::new(),
            source_dir: None,
            output_dir: None,
        };

        assert_eq!(
            resolve_placeholders("{mods}/skse/scripts", &config),
            "/mods/skse/scripts"
        );

        assert_eq!(
            resolve_placeholders("{scripts}/source", &config),
            "/scripts/source"
        );

        assert_eq!(
            resolve_placeholders("{game}-{name}", &config),
            "sse-TestMod"
        );
    }

    #[test]
    fn test_default_template_is_valid_toml() {
        let template = default_config_template();
        let result: Result<ScribeConfig, _> = toml::from_str(&template);
        assert!(result.is_ok(), "Default template should be valid TOML");
    }
}
