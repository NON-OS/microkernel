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

//! The version 1 layouts of the image and its chunk list. All integers are
//! little-endian and every offset is from the object's start.

pub const IMAGE_MAGIC: [u8; 8] = *b"NONOSAI1";
pub const LIST_MAGIC: [u8; 8] = *b"NONOSAC1";
pub const VERSION: u16 = 1;
pub const HEADER_LEN: usize = 64;
pub const MEMBER_LEN: usize = 64;
/// The chunk list's fixed part, before its digests.
pub const LIST_HEAD: usize = 32;
pub const DIGEST_LEN: usize = 32;
/// The most packages one image carries: the installer's own default bound
/// on a closure (capsule_linux install/limit.rs).
pub const MAX_MEMBERS: usize = 1024;
/// The largest image, header to last byte.
pub const MAX_IMAGE: u64 = 1 << 30;
/// The most a member's tar may hold once decompressed: what the device's xz
/// decoder produces in one call (nonos_xz MAX_OUT).
pub const MAX_RAW: u64 = 256 << 20;
/// A member's name, an Alpine package name, at its longest.
pub const NAME_MAX: usize = 64;
/// Chunk sizes, powers of two between these.
pub const CHUNK_MIN: u32 = 64 << 10;
pub const CHUNK_MAX: u32 = 16 << 20;

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

/// Whether `b[from..to]` is all zero.
pub fn zero(b: &[u8], from: usize, to: usize) -> bool {
    b[from..to].iter().all(|x| *x == 0)
}
