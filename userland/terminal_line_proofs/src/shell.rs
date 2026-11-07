// NONOS Operating System (AGPL-3.0-or-later)
//! How a typed line becomes words, statements and redirects, and "did you mean".

// The tokenizer, the `;`/`&&`/`&` split, and the redirect plan the dispatch
// and a tool's job both read.
#[path = "../../nonos_shell/src/parse/mod.rs"]
pub mod parse;
#[path = "../../nonos_shell/src/redirect.rs"]
pub mod redirect;
#[path = "../../nonos_shell/src/statements.rs"]
pub mod statements;
#[path = "../../nonos_shell/src/tool_admit.rs"]
pub mod tool_admit;

#[path = "../../nonos_shell/src/suggest.rs"]
pub mod suggest;

#[cfg(test)]
mod redirect_tests;
#[cfg(test)]
mod words_tests;
#[cfg(test)]
mod suggest_tests;
