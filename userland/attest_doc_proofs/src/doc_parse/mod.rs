// NONOS Operating System (AGPL-3.0-or-later)
//! The parser's own module shape, so the shipping files compile unchanged.

#[allow(dead_code)]
#[path = "../../../capsule_about/src/about/data/doc_parse/layout.rs"]
pub mod layout;
#[allow(dead_code)]
#[path = "../../../capsule_about/src/about/data/doc_parse/types.rs"]
pub mod types;
#[allow(dead_code)]
#[path = "../../../capsule_about/src/about/data/doc_parse/parse.rs"]
pub mod parse;

pub use parse::parse;
pub use types::Doc;
