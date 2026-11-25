// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The proofs a family has had verified, by what was verified
//! (attest_seen_key.rs): pure, so the host proofs hold the rule as it ships
//! (attest_seen.rs keeps one table).
//!
//! The kernel's answer to a proof is a function of the image's bytes, the
//! trailer, the certificate and manifest when there are any, and the roots it
//! holds. The key (attest_seen_key.rs) binds exactly those bytes, hashed in
//! this capsule on every ask, from the bytes in hand on that ask: no path, no
//! time, no store generation, and nothing about who may write the tree they
//! came from, since a rewritten file is other bytes and another key. A hit
//! spares the kernel's hash of the image; it never spares the hash that
//! shows these bytes are the bytes the kernel accepted, and the caller maps
//! the very bytes that hash was taken over (call/mem/map_exec.rs).
//!
//! A family's seal lists each measurement once
//! (src/process/foreign/seal/family_rows.rs), so the second ask would have
//! added nothing to it; a new family starts with no answers, so each
//! family's seal still holds every measurement its guests ran under.
//!
//! Accepted lag, as the kernel's own admission cache has it for capsules
//! (src/security/capsule_attest/path.rs:50, `known` in verify_against): a
//! root withdrawn while a family runs is not seen by that family's kept
//! answers (userland/capsule_linux/src/linux/attest.rs:43, `known`) until the
//! family ends; every new family asks the kernel again.

use alloc::vec::Vec;

/// Past this many, answers are no longer kept and each proof goes to the
/// kernel, as every one did before: a bound on the table, never a refusal.
pub const MAX_SEEN: usize = 512;

/// The answers one family has had: each key with what the kernel said of it
/// (`S`, its verification summary).
pub struct Seen<S> {
    answers: Vec<([u8; 32], S)>,
}

impl<S: Copy> Seen<S> {
    pub const fn new() -> Self {
        Self { answers: Vec::new() }
    }

    /// The kernel's earlier answer for `key` in this family, if it gave one.
    pub fn known(&self, key: &[u8; 32]) -> Option<S> {
        self.answers.iter().find(|(k, _)| k == key).map(|(_, s)| *s)
    }

    /// A proof the kernel accepted. Only acceptances are kept: a refusal is
    /// asked again, so nothing here can turn a refusal into an acceptance.
    pub fn accepted(&mut self, key: [u8; 32], summary: S) {
        if self.answers.len() < MAX_SEEN && self.known(&key).is_none() {
            self.answers.push((key, summary));
        }
    }

    /// A new family is about to begin: none of these answers are its own.
    pub fn forget(&mut self) {
        self.answers.clear();
    }
}
