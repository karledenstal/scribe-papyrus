use anyhow::{Context, Result};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::bundled;
use crate::config::{self, ScribeConfig};

/// Run the compile command
pub fn run(path: PathBuf, check: bool, config_path: Option<PathBuf>) -> Result<()> {
    // 1. Find and load config
    let config_file = config::find_config(&env::current_dir()?, config_path)?;
    let mut config = config::load_config(&config_file)?;

    // 2. If using bundled compiler, ensure it's extracted
    let bundled_path = bundled::bundled_compiler_path()?;
    if config.compiler == bundled_path || !config.compiler.exists() {
        config.compiler = bundled::ensure_bundled_compiler()?;
    }

    // 3. Determine what to compile
    let files_to_compile = resolve_files_to_compile(&path, &config)?;

    // 4. Compile each file
    for file in &files_to_compile {
        println!("Compiling {}...", file.display());

        let success = compile_single_file(&config, file, check)?;

        if !success {
            eprintln!("Compilation failed for {}", file.display());
            std::process::exit(1);
        }
    }

    println!("✓ Successfully compiled {} file(s)", files_to_compile.len());
    Ok(())
}

/// Resolve which files to compile based on the provided path
fn resolve_files_to_compile(path: &Path, config: &ScribeConfig) -> Result<Vec<PathBuf>> {
    // Case 1: Path is a single .psc file
    if path.is_file() && path.extension() == Some(OsStr::new("psc")) {
        return Ok(vec![path.to_path_buf()]);
    }

    // Case 2: Path is "." - use sourceDir from config
    let search_dir = if path == Path::new(".") {
        let source_dir = config
            .source_dir
            .as_ref()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("./src/scripts/source"));

        let resolved = config::resolve_placeholders(&source_dir.to_string_lossy(), config);
        PathBuf::from(resolved)
    } else if path.is_dir() {
        // Case 3: Path is a directory
        path.to_path_buf()
    } else {
        anyhow::bail!("Path '{}' is not a valid file or directory", path.display());
    };

    // Recursively find all .psc files
    let mut psc_files = Vec::new();
    find_psc_files_recursive(&search_dir, &mut psc_files)?;

    if psc_files.is_empty() {
        anyhow::bail!("No .psc files found in {}", search_dir.display());
    }

    // Sort for consistent ordering
    psc_files.sort();

    Ok(psc_files)
}

/// Recursively find all .psc files in a directory
fn find_psc_files_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    if !dir.is_dir() {
        anyhow::bail!("'{}' is not a directory", dir.display());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recurse into subdirectory
            find_psc_files_recursive(&path, files)?;
        } else if path.extension() == Some(OsStr::new("psc")) {
            // Found a .psc file
            files.push(path);
        }
    }

    Ok(())
}

/// Compile a single .psc file
fn compile_single_file(config: &ScribeConfig, file: &Path, check: bool) -> Result<bool> {
    let args = build_compiler_args(config, file, check)?;

    let status = Command::new(&config.compiler)
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context(format!("Failed to execute compiler for {}", file.display()))?;

    Ok(status.success())
}

/// Build the compiler arguments
fn build_compiler_args(config: &ScribeConfig, file: &Path, check: bool) -> Result<Vec<String>> {
    let mut args = vec!["compile".to_string()];

    // Add -nocache flag
    args.push("-nocache".to_string());

    // Add -check flag if requested
    if check {
        args.push("-check".to_string());
    }

    // Add scripts header path: -h "scripts_path"
    args.push("-h".to_string());
    args.push(config.scripts.display().to_string());

    // Add each dependency as header path: -h "dep_path"
    for (_name, dep_path) in &config.dependencies {
        let resolved = config::resolve_placeholders(dep_path, config);
        args.push("-h".to_string());
        args.push(resolved);
    }

    // Input file: -i "file.psc"
    args.push("-i".to_string());
    args.push(file.display().to_string());

    // Output directory: -o "output_dir"
    let output_dir = config
        .output_dir
        .as_ref()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "./src/scripts".to_string());
    let resolved_output = config::resolve_placeholders(&output_dir, config);
    args.push("-o".to_string());
    args.push(resolved_output);

    Ok(args)
}
