//! loader.rs
//!
//! Code for loading and fetching snippets. The snippets are parsed as JSONC,
//! so that they can be inserted into the user's settings.json.
use std::fs;

use reqwest::Url;

/// Fetch the given snippet from the web.
fn fetch_web_snippet(url: Url) -> Result<String, String> {
    todo!()
}

/// Load the given snippet from a file
fn load_file_snippet(path: &str) -> Result<String, String> {
    let bytes = fs::read(path)
        .or(Err("Unable to read file".to_owned()))
        ?;
    String::try_from(bytes)
        .or(Err("File is not a valid encoding".to_owned()))
}

/// Given a snippet path/URL, load and parse it
pub fn load_snippet(snippet: &str) -> Result<String, String> {
    if let Ok(url) = Url::parse(snippet) {
        fetch_web_snippet(url)
    } else {
        load_file_snippet(snippet)
    }
}
