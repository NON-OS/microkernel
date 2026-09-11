// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the TPM machine-key derivation. Includes the kernel's own
//! wire code via `#[path]` so the bytes under test are the bytes that ship.
//!
//! The derivation is six commands whose framing has to be right to the byte,
//! and the TPM's answer to a wrong byte is a response code with no further
//! help. These pin each command against the specification, and, where a
//! software TPM is installed, run the sequence for real.

extern crate alloc;

/// The kernel's module tree, enough of it for `crate::security::tpm::...` to
/// resolve to the shipping files.
pub mod security;

pub use security::tpm::machine_key;
