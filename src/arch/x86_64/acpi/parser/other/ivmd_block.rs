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

//! One IVMD block, pure. An exclusion range is kept as a read and
//! write unity range, as Linux has since 5.5.

pub(super) const ALL: u8 = 0x20;
pub(super) const ONE: u8 = 0x21;
pub(super) const RANGE: u8 = 0x22;
const UNITY: u8 = 1 << 0;
const READ: u8 = 1 << 1;
const WRITE: u8 = 1 << 2;
const EXCLUSION: u8 = 1 << 3;
const PAGE: u64 = 4096;

/// Requester ids `first..=last` keep `[start, end)`, page aligned outward.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Reserved {
    pub first: u16,
    pub last: u16,
    pub start: u64,
    pub end: u64,
    pub read: bool,
    pub write: bool,
}

/// One 32-byte IVMD block's range, or None when it names nothing usable.
pub(super) fn block(kind: u8, b: &[u8]) -> Option<Reserved> {
    let word = |i: usize| u16::from_le_bytes([b[i], b[i + 1]]);
    let quad = |i: usize| u64::from_le_bytes(b[i..i + 8].try_into().unwrap_or([0; 8]));
    let flags = b[1];
    let exclusion = flags & EXCLUSION != 0;
    if flags & UNITY == 0 && !exclusion {
        return None;
    }
    let (first, last) = match kind {
        ALL => (0, u16::MAX),
        ONE => (word(4), word(4)),
        _ => (word(4), word(6)),
    };
    let (start, len) = (quad(16), quad(24));
    let end = start.checked_add(len)?.checked_add(PAGE - 1)? & !(PAGE - 1);
    let start = start & !(PAGE - 1);
    if len == 0 || last < first || end <= start {
        return None;
    }
    let (read, write) = (exclusion || flags & READ != 0, exclusion || flags & WRITE != 0);
    if !read && !write {
        return None;
    }
    Some(Reserved { first, last, start, end, read, write })
}
