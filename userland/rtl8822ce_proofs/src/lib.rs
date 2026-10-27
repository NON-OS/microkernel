// NONOS Operating System (AGPL-3.0-or-later)

//! Host proofs for the RTL8822CE driver: its source, mounted as the capsule
//! mounts it (`mounts`, `serve`), driven against a modelled register file
//! (`model`).

extern crate alloc;

#[path = "mounts.rs"]
mod mounts;
pub use mounts::*;

// The serve stage at the crate's root, where its `pub(in crate::serve)`
// items name it, mirrored file by file so the radio's proofs sit inside it.
pub mod serve;

#[cfg(test)]
mod model;
#[cfg(test)]
mod tests;
