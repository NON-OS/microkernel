// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the attestation document parser. Includes the real capsule
//! source via `#[path]` so the code under test is the code that ships.
//!
//! A parser that decides whether a signed statement about the machine is well
//! formed is the wrong place to be lenient: a document accepted on a length it
//! did not check is a document an attacker gets to shape. These drive it with
//! the malformed cases rather than the happy one.

/// The real parser, unchanged.
pub mod doc_parse;

#[cfg(test)]
mod parse_tests;
