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

//! Reading one, strictly.

use super::layout::{ATTEST_LEN_AT, COMPLETE_AT, COUNT_AT, MAGIC, ROOT_AT, VERSION};
use super::types::Doc;

// Strict: every length is checked against what is actually there before it is
// used. A verifier that parses a document it half understands is worse than one
// that rejects it, and that rule applies to the machine reading its own.
pub fn parse(b: &[u8], sent: &[u8; 32]) -> Option<Doc> {
    if b.len() < ATTEST_LEN_AT + 4 {
        return None;
    }
    if &b[0..8] != MAGIC || be32(b, 8)? != VERSION {
        return None;
    }

    let mut registry_root = [0u8; 32];
    registry_root.copy_from_slice(b.get(ROOT_AT..ROOT_AT + 32)?);
    let challenge_echoed = b.get(12..12 + 32)? == sent;

    let attest_len = be32(b, ATTEST_LEN_AT)?;
    let sig_len_at = ATTEST_LEN_AT + 4 + attest_len as usize;
    let signature_len = be32(b, sig_len_at)?;
    // The document must end exactly where its own lengths say it does.
    if sig_len_at + 4 + signature_len as usize != b.len() {
        return None;
    }

    Some(Doc {
        registry_root,
        capsule_count: be32(b, COUNT_AT)?,
        registry_complete: *b.get(COMPLETE_AT)? == 1,
        challenge_echoed,
        attest_len,
        signature_len,
    })
}

fn be32(b: &[u8], at: usize) -> Option<u32> {
    let s = b.get(at..at + 4)?;
    Some(u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
}
