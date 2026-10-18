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

//! The GEOMETRY reply body.

use super::limits::GEOMETRY_LEN;

/// The flags: bit 0 a removable medium, bit 1 one that may not be written,
/// bit 2 a medium present. A fixed disk is only present.
pub const REMOVABLE: u32 = 1 << 0;
pub const READ_ONLY: u32 = 1 << 1;
pub const MEDIA_PRESENT: u32 = 1 << 2;

/// Write the body into `out[..GEOMETRY_LEN]`, little endian in wire order:
/// lba_count u64, lba_bytes u32, max_transfer u32, flags u32.
pub fn encode_geometry(
    out: &mut [u8],
    lba_count: u64,
    lba_bytes: u32,
    max_transfer: u32,
    flags: u32,
) {
    let out = &mut out[..GEOMETRY_LEN];
    out[0..8].copy_from_slice(&lba_count.to_le_bytes());
    out[8..12].copy_from_slice(&lba_bytes.to_le_bytes());
    out[12..16].copy_from_slice(&max_transfer.to_le_bytes());
    out[16..20].copy_from_slice(&flags.to_le_bytes());
}
