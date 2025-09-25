//! apply.rs
//!
//! Code for applying snippets
use jsonc_parser::{ParseOptions, cst::CstRootNode};

/// Given source settings and a snippet, apply the snippet to the source
/// settings, returning a String containing the new settings contents.
pub fn apply(src: &str, snippet: &str) -> Result<String, String> {
    let mut src_node = CstRootNode::parse(src, &ParseOptions::default())
        .or(Err("Unable to parse settings JSON".to_owned()))?;

    let snippet_node = CstRootNode::parse(snippet, &ParseOptions::default())
        .or(Err("Unable to parse snippet JSON".to_owned()))?;

    todo!()
}
