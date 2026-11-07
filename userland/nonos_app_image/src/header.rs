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

//! The image's 64-byte header: how many members, how long their names are
//! together, and the image's whole length.

use crate::format::{u16_at, u32_at, u64_at, zero};
use crate::format::{HEADER_LEN, IMAGE_MAGIC, MAX_IMAGE, MAX_MEMBERS, MEMBER_LEN, NAME_MAX};
use crate::refused::Refused;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
    pub count: usize,
    pub names_len: usize,
    pub total: u64,
}

impl Header {
    /// Where the member table ends and the names start.
    pub fn names_at(&self) -> usize {
        HEADER_LEN + self.count * MEMBER_LEN
    }

    /// Where the names end and the members' bytes start.
    pub fn data_at(&self) -> usize {
        self.names_at() + self.names_len
    }
}

/// `b` holds at least the header.
pub fn header(b: &[u8]) -> Result<Header, Refused> {
    if b.len() < HEADER_LEN || b[..8] != IMAGE_MAGIC {
        return Err(Refused::Magic);
    }
    let reserved = u16_at(b, 10) == 0 && u16_at(b, 14) == 0 && u32_at(b, 20) == 0;
    if u16_at(b, 8) != crate::format::VERSION || !reserved || !zero(b, 32, HEADER_LEN) {
        return Err(Refused::Magic);
    }
    let (count, names_len, total) = (u16_at(b, 12) as usize, u32_at(b, 16) as usize, u64_at(b, 24));
    if count > MAX_MEMBERS || total > MAX_IMAGE || names_len > count * NAME_MAX {
        return Err(Refused::TooLarge);
    }
    let head = Header { count, names_len, total };
    if count == 0 || names_len < count || (head.data_at() as u64) > total {
        return Err(Refused::Layout);
    }
    Ok(head)
}

/// How many bytes from the image's start `Head::parse` needs, read off its
/// first HEADER_LEN bytes: the header, the member table and the names.
pub fn head_len(first: &[u8]) -> Result<usize, Refused> {
    header(first).map(|h| h.data_at())
}
