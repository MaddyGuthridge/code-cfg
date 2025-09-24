//! snippet.rs
//!
//! Code for loading and fetching snippets. The snippets are parsed as JSONC,
//! so that they can be inserted into the user's settings.json.
use jsonc_parser::cst::CstRootNode;

/// Fetch the given snippet from the web.
fn fetch_web_snippet(url: String) -> String {
    todo!()
}

/// Load the given snippet from a file
fn load_file_snippet(path: String) -> String {
    todo!()
}

/// Given a snippet path/URL, load and parse it
pub fn load_snippet(snippet: String) -> Result<CstRootNode, String> {
    todo!()
}
