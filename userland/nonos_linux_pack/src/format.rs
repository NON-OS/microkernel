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

//! The pack's version 1 layout. All integers are little-endian and every
//! offset is from the pack's start.

pub const MAGIC: [u8; 8] = *b"NONOSPK1";
pub const VERSION: u16 = 1;
pub const HEADER_LEN: usize = 64;
pub const ENTRY_LEN: usize = 48;
/// The most entries an index holds; a pack naming more is refused.
pub const MAX_ENTRIES: usize = 65536;
/// The largest pack, header to last byte: one store object (capsule_vfs's
/// MAX_FILE_BYTES). A version that changes it changes `VERSION`.
pub const MAX_PACK: usize = 64 << 20;
/// A name's longest, and a link target's.
pub const NAME_MAX: usize = 255;
pub const TARGET_MAX: usize = 4095;
/// Where the data area starts, and where each file's bytes start in it.
pub const DATA_ALIGN: usize = 4096;
pub const FILE_ALIGN: usize = 64;
/// The digest kept for each file: the first bytes of its BLAKE3.
pub const DIGEST_LEN: usize = 16;

pub fn up(n: usize, to: usize) -> Option<usize> {
    n.checked_add(to - 1).map(|v| v / to * to)
}

pub fn u16_at(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}

pub fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

pub fn u64_at(b: &[u8], at: usize) -> u64 {
    let mut w = [0u8; 8];
    w.copy_from_slice(&b[at..at + 8]);
    u64::from_le_bytes(w)
}
