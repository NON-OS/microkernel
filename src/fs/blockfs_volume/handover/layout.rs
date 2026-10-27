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
//! format does, clears the record with the rest of the sector. Pure.

use super::super::key_header::{Keyed, Sealed};
use super::super::key_seal::encode_key_header;

pub(super) const MAGIC: [u8; 8] = *b"NONOSHO1";
const AT: usize = 128;
const END: usize = AT + 8 + 12 + 48;
/// Way 3's bytes: magic, way, the bound zeros, its nonce, key and tag.
const WAY3_END: usize = 116;

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

/// What the first boot under a new bootloader writes, in order, once the
/// record opened the volume: way 3 wrapped again under this boot's key with
/// the record kept, then the same without it. Each is read back before the
/// next is written, so the disk holds a header some boot opens throughout:
/// the old wrap and the record, the new wrap and the record, the new wrap.
pub(super) fn moved(s: &[u8; 512], new: &Sealed) -> ([u8; 512], [u8; 512]) {
    let mut kept = *s;
    kept[..WAY3_END].copy_from_slice(&encode_key_header(&Keyed::Approved(*new))[..WAY3_END]);
    (kept, put(&kept, None))
}
