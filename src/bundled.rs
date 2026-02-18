use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

// Embed the papyrus compiler binary at compile time
static PAPYRUS_BINARY: &[u8] = include_bytes!("../bin/papyrus");

/// Get the path where the bundled compiler should be installed
pub fn bundled_compiler_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;

    Ok(home.join(".scribe").join("bin").join("papyrus"))
}

/// Ensure the bundled compiler is extracted and ready to use
/// Returns the path to the compiler
pub fn ensure_bundled_compiler() -> Result<PathBuf> {
    let compiler_path = bundled_compiler_path()?;

    // If compiler already exists and matches embedded version, skip extraction
    if compiler_path.exists() {
        // Verify it's executable and roughly the right size
        let metadata = fs::metadata(&compiler_path)?;
        if metadata.len() == PAPYRUS_BINARY.len() as u64 {
            return Ok(compiler_path);
        }
    }

    // Extract the compiler
    extract_bundled_compiler(&compiler_path)?;

    Ok(compiler_path)
}

/// Extract the bundled compiler to the specified path
fn extract_bundled_compiler(dest_path: &PathBuf) -> Result<()> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)
            .context(format!("Failed to create directory: {}", parent.display()))?;
    }

    // Write the embedded binary to disk
    let mut file = fs::File::create(dest_path)
        .context(format!("Failed to create file: {}", dest_path.display()))?;

    file.write_all(PAPYRUS_BINARY)
        .context("Failed to write compiler binary")?;

    // Make it executable on Unix systems
    #[cfg(unix)]
    {
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o755); // rwxr-xr-x
        fs::set_permissions(dest_path, perms)?;
    }

    println!(
        "✓ Extracted bundled Papyrus compiler to {}",
        dest_path.display()
    );

    Ok(())
}

/// Get the default compiler path string for config template
pub fn default_compiler_path() -> String {
    bundled_compiler_path()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "./bin/papyrus".to_string())
}
