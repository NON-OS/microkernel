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

//! The role table's anchor as the operator puts it on the machine: the key
//! id `nsh-roles table` prints, in hex, with or without a newline, or the
//! same 32 bytes raw. Anything else is no anchor. Pure, so nsh_front_proofs
//! holds it.

/// The anchor's length.
pub const ANCHOR_LEN: usize = 32;
/// The longest file an anchor is read from: 64 hex digits and a newline.
pub const ANCHOR_FILE_MAX: usize = 2 * ANCHOR_LEN + 1;

/// The anchor `file` holds.
pub fn anchor(file: &[u8]) -> Option<[u8; ANCHOR_LEN]> {
    if let Ok(raw) = <[u8; ANCHOR_LEN]>::try_from(file) {
        return Some(raw);
    }
    let text = file.strip_suffix(b"\n").unwrap_or(file);
    if text.len() != 2 * ANCHOR_LEN {
        return None;
    }
    let mut out = [0u8; ANCHOR_LEN];
    for (o, pair) in out.iter_mut().zip(text.chunks_exact(2)) {
        *o = (digit(pair[0])? << 4) | digit(pair[1])?;
    }
    Some(out)
}

fn digit(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}
