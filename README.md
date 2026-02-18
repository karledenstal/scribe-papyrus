# Scribe Papyrus

A command-line wrapper for the Papyrus script compiler with an embedded compiler binary.

Scribe makes it easy to compile Papyrus scripts for Skyrim and other Creation Engine games without manual setup. The Papyrus compiler is bundled directly in the binary - no additional downloads required!

## Features

- **Embedded Compiler**: russo-2025's Papyrus compiler bundled directly in the binary
- **Zero Configuration**: Compiler extracts automatically on first use
- **Batch Compilation**: Compile all scripts with `scribe .`
- **Project Configuration**: TOML-based config with placeholder support
- **Syntax Checking**: Validate scripts without compilation using `--check`
- **Recursive Discovery**: Automatically finds all `.psc` files in subdirectories

## Installation

```bash
cargo install scribe-papyrus
```

## Quick Start

1. **Initialize a new project**:

   ```bash
   scribe init
   ```

2. **Edit `scribe.toml`** with your project paths:

   ```toml
   name = "My Awesome Mod"
   mods = "/path/to/mods/folder"
   game = "sse"
   scripts = "/path/to/Data/Scripts/Source"
   compiler = "/home/username/.scribe/bin/papyrus"  # Default bundled compiler
   
   [dependencies]
   skse = "{mods}/skse/scripts/source"
   skyui = "{mods}/skyui/scripts/source"
   
   sourceDir = "./src/scripts/source"
   outputDir = "./src/scripts"
   ```

3. **Compile your scripts**:

   ```bash
   # Compile all scripts in sourceDir
   scribe .
   
   # Compile a single file
   scribe src/MyScript.psc
   
   # Compile a specific directory
   scribe src/scripts/source/quests
   
   # Syntax check only
   scribe . --check
   ```

## Usage

### Commands

- `scribe init` - Create a new `scribe.toml` configuration file
- `scribe <path>` - Compile Papyrus script(s)
  - `<path>` can be:
    - `.` to compile all scripts in `sourceDir`
    - A single `.psc` file
    - A directory to compile all `.psc` files in it (recursively)

### Options

- `--check` - Check syntax without generating `.pex` files
- `--config <path>` - Use a custom config file location

### Configuration

The `scribe.toml` file supports:

- **Placeholders**: Use `{mods}`, `{scripts}`, `{game}`, `{name}` in paths
- **Dependencies**: Map dependency names to header directories
- **Custom paths**: Override source and output directories

### Examples

```bash
# Initialize new project
scribe init

# Compile everything
scribe .

# Compile single file
scribe src/scripts/source/MyQuest.psc

# Check syntax only
scribe src/scripts/source/MyQuest.psc --check

# Compile from a subdirectory (searches for scribe.toml in parent dirs)
cd src/scripts
scribe source/MyQuest.psc
```

## How It Works

1. On first run, Scribe extracts the bundled Papyrus compiler to `~/.scribe/bin/papyrus`
2. The compiler is cached and reused for all subsequent compilations
3. Scribe finds your `scribe.toml` by searching upward from the current directory
4. It builds the appropriate compiler arguments and executes the compiler
5. Compilation output is streamed directly to your terminal

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Credits

- Papyrus compiler by [russo-2025](https://github.com/russo-2025)
- Scribe wrapper tool by [Your Name]
