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

//! Whether a disk's GPT lists a partition by its GUID, from the bytes of
//! its header and its entries (UEFI 2.10, 5.3). Pure, so the host proofs
//! mount this file as it is.

const SIGNATURE: &[u8; 8] = b"EFI PART";
const HEADER_LEN: usize = 92;
const ENTRY_MIN: usize = 128;
/// The most entry bytes read: 128 entries of 128 bytes is the usual table,
/// and every table this loader's images and installs write.
pub const ENTRY_BYTES_MAX: usize = 64 << 10;

/// Where the entries lie, as (first LBA, count, size of one), from the
/// header at LBA 1; `None` for no GPT, or a shape this does not read.
pub fn entries(header: &[u8]) -> Option<(u64, usize, usize)> {
    if header.len() < HEADER_LEN || &header[..8] != SIGNATURE {
        return None;
    }
    let lba = u64::from_le_bytes(header[72..80].try_into().ok()?);
    let count = u32::from_le_bytes(header[80..84].try_into().ok()?) as usize;
    let size = u32::from_le_bytes(header[84..88].try_into().ok()?) as usize;
    let fits = count.checked_mul(size).is_some_and(|n| n <= ENTRY_BYTES_MAX);
    let shaped = size >= ENTRY_MIN && size % 8 == 0 && count > 0 && lba >= 2;
    (fits && shaped).then_some((lba, count, size))
}

/// Whether `entries`, each `size` bytes, name `guid` as a partition's own
/// (bytes 16..32 of an entry, as stored). An all-zero GUID names nothing.
pub fn lists(entries: &[u8], size: usize, guid: &[u8; 16]) -> bool {
    *guid != [0; 16] && size >= 32 && entries.chunks_exact(size).any(|e| e[16..32] == *guid)
}
