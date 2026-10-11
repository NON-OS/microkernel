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

use alloc::vec::Vec;

use super::kind::Kind;
use super::parse::{HEAD, MAGIC, MAX_BYTES, SECTION_HEAD, VERSION};
use super::tag::{fits, LAST};

/// A container from `(tag, bytes)` pairs. `None` when the pairs break a rule
/// `parse` enforces, so whatever this writes, `parse` reads.
pub fn encode(kind: Kind, sections: &[(u16, &[u8])]) -> Option<Vec<u8>> {
    let n = u16::try_from(sections.len()).ok()?;
    let mut last = 0u16;
    let mut size = HEAD;
    for &(tag, bytes) in sections {
        if tag <= last || tag > LAST || !fits(tag, bytes.len()) {
            return None;
        }
        u32::try_from(bytes.len()).ok()?;
        size = size.checked_add(SECTION_HEAD + bytes.len())?;
        last = tag;
    }
    if size > MAX_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(size);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&(kind as u16).to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&n.to_le_bytes());
    for &(tag, bytes) in sections {
        out.extend_from_slice(&tag.to_le_bytes());
        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(bytes);
    }
    Some(out)
}
