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

//! Where the handover record lies in the key header's sector, beside way 3:
//!
//!   bytes 128..136  magic "NONOSHO1"
//!         136..148  the nonce the volume key is sealed under
//!         148..196  the volume key and its tag (seal.rs)
//!
//! Way 3 ends at byte 116 (key_header.rs), so writing way 3 alone, as a
//! format does, clears the record with the rest of the sector. The installer
//! reads the same bytes to know whether a disk is ready to update, and its
//! tests read them with this file. Pure, and needs nothing beside it.

pub(super) const MAGIC: [u8; 8] = *b"NONOSHO1";
const AT: usize = 128;
const END: usize = AT + 8 + 12 + 48;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Record {
    pub(super) nonce: [u8; 12],
    pub(super) key_and_tag: [u8; 48],
}

/// The record `s` holds, if it holds one.
pub(super) fn read(s: &[u8; 512]) -> Option<Record> {
    if s[AT..AT + 8] != MAGIC {
        return None;
    }
    let nonce = s[AT + 8..AT + 20].try_into().ok()?;
    Some(Record { nonce, key_and_tag: s[AT + 20..END].try_into().ok()? })
}

/// `s` with `r` in the record's place, or with the place cleared.
pub(super) fn put(s: &[u8; 512], r: Option<&Record>) -> [u8; 512] {
    let mut out = *s;
    out[AT..END].fill(0);
    if let Some(r) = r {
        out[AT..AT + 8].copy_from_slice(&MAGIC);
        out[AT + 8..AT + 20].copy_from_slice(&r.nonce);
        out[AT + 20..END].copy_from_slice(&r.key_and_tag);
    }
    out
}
