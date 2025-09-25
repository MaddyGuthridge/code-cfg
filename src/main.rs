//! Main entrypoint for code-cfg program
mod snippet;

use std::{env, path::PathBuf};

use clap::Parser;

/// Returns the path to the user settings JSON file
///
/// https://code.visualstudio.com/docs/configure/settings#_user-settingsjson-location
fn code_settings_file() -> PathBuf {
    if cfg!(target_os="windows") {
        PathBuf::from(env::var("APPDATA").unwrap())
            .join("Code")
            .join("User")
            .join("settings.json")
    } else if cfg!(target_os="linux") {
        PathBuf::from(env::var("HOME").unwrap())
            .join(".config")
            .join("Code")
            .join("User")
            .join("settings.json")
    } else if cfg!(target_os="macos") {
        PathBuf::from(env::var("HOME").unwrap())
            .join("Library")
            .join("Application Support")
            .join("Code")
            .join("User")
            .join("settings.json")
    } else {
        panic!("Unknown platform, cannot determine Code settings file path")
    }
}

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// The URLs or file paths for settings to apply
    #[arg(required=true)]
    snippets: Vec<String>,

    /// The VS Code settings file to modify
    #[arg(short, long)]
    file: Option<String>,

    /// Whether to accept installing all of the given settings automatically,
    /// as opposed to prompting for every snippet set.
    #[arg(long)]
    all: bool
}

fn main() {
    let cli = Args::parse();

    println!("{cli:?}");
}
