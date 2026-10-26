// NONOS Operating System (AGPL-3.0-or-later)
//! How a typed line becomes words, statements and redirects, and "did you mean".

// The tokenizer, the `;`/`&&`/`&` split, and the redirect plan the dispatch
// and a tool's job both read.
#[path = "../../capsule_terminal/src/command/parse/mod.rs"]
pub mod parse;
#[path = "../../capsule_terminal/src/command/dispatch/redirect.rs"]
pub mod redirect;
#[path = "../../capsule_terminal/src/command/dispatch/statements.rs"]
pub mod statements;
#[path = "../../capsule_terminal/src/command/dispatch/tool_admit.rs"]
pub mod tool_admit;

#[path = "../../capsule_terminal/src/command/suggest.rs"]
pub mod suggest;

#[cfg(test)]
mod redirect_tests;
#[cfg(test)]
mod words_tests;
#[cfg(test)]
mod suggest_tests;
