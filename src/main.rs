//! code-cfg
//! ========
//!
//! A configuration tool to share VS Code settings more easily.
//!
//! This is main.rs, the main entrypoint to the program.
mod apply;
mod loader;

use std::{env, fs, path::PathBuf};

use clap::Parser;

use crate::{apply::apply, loader::load_snippet};

/// Returns the path to the user settings JSON file
///
/// https://code.visualstudio.com/docs/configure/settings#_user-settingsjson-location
fn code_settings_file() -> String {
    if cfg!(target_os = "windows") {
        PathBuf::from(env::var("APPDATA").unwrap())
            .join("Code")
            .join("User")
            .join("settings.json")
            .to_str()
            .unwrap()
            .to_owned()
    } else if cfg!(target_os = "linux") {
        PathBuf::from(env::var("HOME").unwrap())
            .join(".config")
            .join("Code")
            .join("User")
            .join("settings.json")
            .to_str()
            .unwrap()
            .to_owned()
    } else if cfg!(target_os = "macos") {
        PathBuf::from(env::var("HOME").unwrap())
            .join("Library")
            .join("Application Support")
            .join("Code")
            .join("User")
            .join("settings.json")
            .to_str()
            .unwrap()
            .to_owned()
    } else {
        panic!("Unknown platform, cannot determine Code settings file path")
    }
}

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// The URLs or file paths for settings to apply
    #[arg(required = true)]
    snippets: Vec<String>,

    /// The VS Code settings file to modify
    #[arg(short, long)]
    file: Option<String>,

    /// Whether to accept installing all of the given settings automatically,
    /// as opposed to prompting for every snippet set.
    #[arg(long)]
    all: bool,
}

fn main() {
    let cli = Args::parse();

    // Holy moly I love Rust's type system so much!
    let snippets = cli
        .snippets
        .iter()
        .map(|snip| load_snippet(snip))
        .collect::<Result<Vec<String>, _>>()
        .unwrap();

    // Load user's base settings
    let base_settings =
        String::try_from(fs::read(cli.file.unwrap_or_else(|| code_settings_file())).unwrap())
            .unwrap();

    // Now apply all snippets
    let new_settings = snippets.into_iter().fold(base_settings, |settings, snip| {
        apply(&settings, &snip).unwrap()
    });

    println!("{new_settings}");
}
