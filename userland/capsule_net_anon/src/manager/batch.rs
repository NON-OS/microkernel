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

//! Splitting a microdescriptor request into URLs a DirPort accepts.

extern crate alloc;

use alloc::vec::Vec;

use crate::base64_encode::encode;
use crate::directory::authority::AUTHORITIES;
use crate::directory::consensus::Entry;
use crate::directory::fetch::micro_path;

/*
 * A DirPort caps the request line, so the digests are asked for in batches rather
 * than all five thousand at once. Ninety two per request keeps the URL under two
 * kilobytes, which every authority accepts.
 */
pub const PER_REQUEST: usize = 92;

/// How many requests the whole directory needs.
pub fn batch_count(entries: &[Entry]) -> usize {
    entries.len().div_ceil(PER_REQUEST)
}

/*
 * One batch, not all of them. This built every request in the list and returned
 * them together, which was fine while the caller sent the lot in one go. It stopped
 * being fine when the fetches were spread one to a turn: each turn base64 encoded
 * all five thousand digests and allocated fifty five URLs of two kilobytes, to use
 * one and drop the rest. Fifty five turns of that is three hundred thousand
 * encodings for five thousand digests' worth of work.
 */

/// The entries batch `index` covers, which are the only digests its answer may
/// contain. Checking a piece against these rather than the whole consensus is the
pub fn batch_entries(entries: &[Entry], index: usize) -> &[Entry] {
    entries.chunks(PER_REQUEST).nth(index).unwrap_or(&[])
}

/// The request for batch `index`, or `None` past the end of the list.
///
pub fn batch_at(entries: &[Entry], start: usize, index: usize) -> Option<([u8; 4], u16, Vec<u8>)> {
    let chunk = entries.chunks(PER_REQUEST).nth(index)?;
    let mut joined: Vec<u8> = Vec::with_capacity(chunk.len() * 44);
    for entry in chunk {
        if !joined.is_empty() {
            joined.push(b'-');
        }
        joined.extend_from_slice(&encode(&entry.microdesc_digest));
    }
    let at = (start + index) % AUTHORITIES.len();
    let authority = &AUTHORITIES[at];
    Some((authority.address, authority.dir_port, micro_path(&joined)))
}
