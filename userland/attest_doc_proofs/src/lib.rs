// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the attestation document parser. Includes the real capsule
//! source via `#[path]` so the code under test is the code that ships.
//!
//! A parser that decides whether a signed statement about the machine is well
//! formed is the wrong place to be lenient: a document accepted on a length it
//! did not check is a document an attacker gets to shape. These drive it with
//! the malformed cases rather than the happy one.

extern crate alloc;

/// The real parser, unchanged.
pub mod doc_parse;

/// About's live proof board: who was admitted, the headline, its words.
#[cfg(test)]
mod proof_board;

#[cfg(test)]
mod proof_board_tests;

/// The kernel's encoder, unchanged. The tests build every document with it, so
/// a layout the kernel writes and the parser does not read fails here instead
/// of on the machine.
#[cfg(test)]
#[path = "../../../src/security/attest_doc/document.rs"]
mod kernel_document;

#[cfg(test)]
mod parse_tests;

#[cfg(test)]
#[path = "../../../src/security/attest_policy/record.rs"]
mod kernel_policy_record;

#[cfg(test)]
#[path = "../../libc/src/attest_policy.rs"]
mod libc_policy_record;

#[cfg(test)]
mod policy_record_tests;

/// The kernel's `MkBootAttest` encoder and libc's reader, unchanged, held to
/// each other the same way.
#[cfg(test)]
#[path = "../../../src/security/boot/loader_check/record.rs"]
mod kernel_boot_record;

#[cfg(test)]
#[path = "../../libc/src/boot_attest/record.rs"]
mod libc_boot_record;

#[cfg(test)]
mod boot_record_tests;
