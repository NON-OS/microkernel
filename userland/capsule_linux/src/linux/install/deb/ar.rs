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

//! The ar archive a .deb is: a magic line, then members each behind a
//! 60-byte header, padded to an even offset.

use alloc::vec::Vec;

const MAGIC: &[u8] = b"!<arch>\n";
const HEADER: usize = 60;

/// Every member's name and bytes, or None if any header is malformed.
pub fn members(d: &[u8]) -> Option<Vec<(&[u8], &[u8])>> {
    let mut at = MAGIC.len();
    if !d.starts_with(MAGIC) {
        return None;
    }
    let mut out = Vec::new();
    while at < d.len() {
        let h = d.get(at..at + HEADER)?;
        if &h[58..60] != b"`\n" {
            return None;
        }
        let name = trim(&h[..16]);
        let name = name.strip_suffix(b"/").unwrap_or(name);
        let size: usize = core::str::from_utf8(trim(&h[48..58])).ok()?.parse().ok()?;
        let end = (at + HEADER).checked_add(size)?;
        out.push((name, d.get(at + HEADER..end)?));
        at = end + (size & 1);
    }
    Some(out)
}

fn trim(field: &[u8]) -> &[u8] {
    let end = field.iter().rposition(|&b| b != b' ').map_or(0, |i| i + 1);
    &field[..end]
}
