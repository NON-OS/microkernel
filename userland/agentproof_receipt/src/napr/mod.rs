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
//! NAPR v0, the receipt of one boxed run (letter agent-02). Big-endian; after
//! the 6-byte head every field is a u32 length and its bytes, in this order:
//!
//! | field | bytes | what |
//! |---|---|---|
//! | record | 240 | NFSR v1, the kernel's sealed record of the run |
//! | box | 1..=255 | the box's `--box` word; its hash is recomputed |
//! | app policy | 256 | the program's NLXPOL01 policy; its app_id is recomputed |
//! | launcher | 32 | the measurement of the capsule that asked; one of the entries |
//! | doc | ..=DOC_MAX | NONOSATT v5 for this receipt's challenge, its unsigned parts cleared |
//! | entries | 45 each, 1..=4096 | MkAttestEntries: what ran; they fold to the doc's signed root |
//! | capsules | 65 each, 0..=4096 | PCR 14's events from the doc's log; they replay to its PCR 14 |
//!
//! One receipt has one encoding: a reader refuses another version, a short or
//! long field, and any byte past the capsules.

pub mod capsules;
mod cursor;
mod layout;
mod le;
#[cfg(feature = "verify")]
pub mod doc_form;
pub mod doc_clear;
pub mod read;
pub mod write;

pub use layout::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Receipt<'a> {
    pub record: &'a [u8; RECORD],
    pub box_word: &'a [u8],
    pub policy: &'a [u8; POLICY],
    pub launcher: &'a [u8; LAUNCHER],
    pub doc: &'a [u8],
    /// Every capsule the machine ran, as the doc's registry root folds them.
    pub entries: &'a [u8],
    /// Every capsule PCR 14 records, as the doc's quote signed it.
    pub capsules: &'a [u8],
    /// The bytes the challenge covers.
    pub bound: &'a [u8],
}

pub fn challenge(bound: &[u8]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(CHALLENGE_DOMAIN).update(bound);
    *h.finalize().as_bytes()
}
