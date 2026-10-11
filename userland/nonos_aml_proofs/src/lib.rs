// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the AML interpreter. The modules below are the shipping
//! sources of `userland/nonos_aml`, at the `crate::` paths they name each
//! other by.

extern crate alloc;

#[path = "../../nonos_aml/src/arena/mod.rs"]
pub mod arena;
#[path = "../../nonos_aml/src/block.rs"]
pub mod block;
#[path = "../../nonos_aml/src/error.rs"]
pub mod error;
#[path = "../../nonos_aml/src/grammar/mod.rs"]
pub mod grammar;
#[path = "../../nonos_aml/src/op/mod.rs"]
pub mod op;
#[path = "../../nonos_aml/src/parse/mod.rs"]
pub mod parse;
#[path = "../../nonos_aml/src/shape/mod.rs"]
pub mod shape;

#[cfg(test)]
mod enc;
#[cfg(test)]
mod flat;
#[cfg(test)]
mod tests;
