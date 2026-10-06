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

//! The package store's container, from `STORE_BASE_LBA` on.
//!
//! ```text
//!   header, 32 bytes   0..8 magic, 8..12 version, 12..16 entry count (LE)
//!   entry, 128 bytes   0..96 path, NUL padded; 96..104 payload offset in
//!                      bytes from the start of the disk; 104..112 length;
//!                      112..128 digest16 of the payload, zero for none
//! ```
//!
//! Entries follow the header. Payloads start on sector boundaries.

use crate::places::SECTOR_SIZE;

pub const STORE_MAGIC: [u8; 8] = *b"NONOSTR1";
pub const STORE_VERSION: u32 = 1;
pub const HEADER_LEN: usize = 32;
pub const ENTRY_LEN: usize = 128;
pub const NAME_LEN: usize = 96;

/// Entries a table may hold. The table is decoded into a heap vector and
/// every payload byte is still bounded by `MAX_TOTAL_BYTES`, so this only
/// sizes the table: 512 entries are 64 KiB. It was 128 until the image
/// carried every Linux tool, Perl's and Tcl's libraries a file each, some
/// 220 entries, which would have left a person's own files 36.
pub const MAX_ENTRIES: usize = 512;

/// Payload bytes vfs loads at most, summed over the entries, so a hostile
/// table cannot exhaust the heap vfs loads the store into. 96 MiB, the bound
/// tools/nonos-store-pack writes to; the installer sizes its heap from it.
/// A full table and every payload padded to a sector still end below the
/// disk plan (tests/container_format.rs). It was 48 MiB until the image
/// carried a Linux userland and the Qwen runner, then 60 MiB until it
/// carried every Linux tool, which run with no network and no install.
pub const MAX_TOTAL_BYTES: u64 = 96 * 1024 * 1024;

/// Store entries under this prefix are streamed: vfs keeps only their place
/// in the table and serves each read from the device, so none of their bytes
/// stay in its memory. The wallpaper collection is the one such entry; the
/// catalog reads the wallpapers kept at setup out of it, one at a time, and
/// holds each to a SHA-256 pinned in its own signed image.
pub const STREAMED_PREFIX: &str = "/Wallpapers/";

/// Streamed payload bytes a store may carry, apart from `MAX_TOTAL_BYTES`:
/// they bound the disk, not the heap, since vfs never loads them. The
/// wallpaper collection is 11.6 MiB; 20 MiB is what the store's window has
/// left beside the loaded budget and a full table (tests/container_format.rs).
pub const STREAMED_MAX_BYTES: u64 = 20 * 1024 * 1024;

/// Whether the entry at `name` is streamed rather than loaded.
pub fn streamed(name: &str) -> bool {
    name.starts_with(STREAMED_PREFIX)
}

/// The header and a full table, in whole sectors. vfs appends its first
/// payload past this, so a table that grows to `MAX_ENTRIES` never runs
/// into a payload.
pub const TOC_SPAN: usize =
    (HEADER_LEN + ENTRY_LEN * MAX_ENTRIES).div_ceil(SECTOR_SIZE) * SECTOR_SIZE;

/// The one test for a path in the table, so a name that writes is a name
/// that decodes: vfs leaves out a descriptor whose name fails it.
///
/// A name is an absolute path as vfs's `normalize` leaves one: a `/`, then
/// components that are neither empty, `.` nor `..`, with no trailing `/`,
/// in printable ASCII, and short enough for the field. Every lookup goes
/// through `normalize`, so any other spelling names a file nothing can
/// open; and one under `/capsules/` that climbs out with `..` would pass
/// the capsule tree's prefix test while naming something outside it.
pub fn valid_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    let Some(rest) = bytes.strip_prefix(b"/") else {
        return false;
    };
    bytes.len() <= NAME_LEN
        && bytes.iter().all(|b| (b' '..=b'~').contains(b))
        && rest.split(|&b| b == b'/').all(|part| !matches!(part, b"" | b"." | b".."))
}
