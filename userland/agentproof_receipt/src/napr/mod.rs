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
//! | launcher | 32 | the capsule that asked for the run; zero until named (gap 5) |
//! | doc | ..=DOC_MAX | NONOSATT v5 for this receipt's challenge, its unsigned parts cleared |
//!
//! One receipt has one encoding: a reader refuses another version, a short or
//! long field, and any byte past the doc.

mod cursor;
pub mod doc_form;
pub mod read;
pub mod write;

pub const MAGIC: &[u8; 4] = b"NAPR";
pub const VERSION: u16 = 0;
pub const HEAD: usize = 6;
pub const RECORD: usize = 240;
pub const BOX_MAX: usize = 255;
pub const POLICY: usize = 256;
pub const LAUNCHER: usize = 32;
/// nsh's cap on a version 5 document (docs/design/nsh-v1-wire.md).
pub const DOC_MAX: usize = 1088 * 1024;

/// The challenge the doc answers: BLAKE3 over this domain and every byte of
/// the receipt before the doc's length.
pub const CHALLENGE_DOMAIN: &[u8] = b"NONOS-AGENTPROOF-RECEIPT-v0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Receipt<'a> {
    pub record: &'a [u8; RECORD],
    pub box_word: &'a [u8],
    pub policy: &'a [u8; POLICY],
    pub launcher: &'a [u8; LAUNCHER],
    pub doc: &'a [u8],
    /// The bytes the challenge covers.
    pub bound: &'a [u8],
}

pub fn challenge(bound: &[u8]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(CHALLENGE_DOMAIN).update(bound);
    *h.finalize().as_bytes()
}
