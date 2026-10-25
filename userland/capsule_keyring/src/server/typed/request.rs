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

//! The request, as the wire carries it (little-endian):
//! pid u32 | key u32 | entry u16 | reserved u16 | chain_id u64 | n u8 |
//! n x 32-byte field words. The caller gives the fields' values and nothing
//! else: no type, no domain, no contract.

/// Bytes before the first field word.
pub const HDR: usize = 4 + 4 + 2 + 2 + 8 + 1;
/// No entry has more fields than this; a larger count is refused unread.
pub const MAX_FIELDS: usize = 16;

#[derive(Debug, PartialEq, Eq)]
pub struct Typed<'a> {
    pub pid: u32,
    pub key: u32,
    pub entry: u16,
    pub chain: u64,
    /// n x 32 bytes, exactly.
    pub words: &'a [u8],
}

pub fn parse(p: &[u8]) -> Option<Typed<'_>> {
    if p.len() < HDR || p[10..12] != [0, 0] {
        return None;
    }
    let n = p[HDR - 1] as usize;
    if n > MAX_FIELDS || p.len() != HDR + n * 32 {
        return None;
    }
    let u32_at = |at: usize| u32::from_le_bytes([p[at], p[at + 1], p[at + 2], p[at + 3]]);
    let mut chain = [0u8; 8];
    chain.copy_from_slice(&p[12..20]);
    Some(Typed {
        pid: u32_at(0),
        key: u32_at(4),
        entry: u16::from_le_bytes([p[8], p[9]]),
        chain: u64::from_le_bytes(chain),
        words: &p[HDR..],
    })
}
