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

//! The first patch file layout. After the signature "Realtech" come the
//! firmware version (4 octets) and the patch count n (2), then three tables
//! of n entries: chip ids (2 octets each), patch lengths (2) and patch
//! offsets (4), all little-endian. The patch for a ROM is the one whose
//! chip id is the ROM version plus one. Its last four octets are left for
//! the firmware version, which this host writes there.

use super::refused::Layout;

pub const SIGNATURE: &[u8; 8] = b"Realtech";
/// Signature, firmware version and patch count.
pub const HEADER: usize = 14;

pub struct Patch<'a> {
    /// The patch without its last four octets.
    pub body: &'a [u8],
    pub version: u32,
}

pub fn patch_for(file: &[u8], rom: u8) -> Result<Patch<'_>, Layout> {
    if file.len() < HEADER || &file[..8] != SIGNATURE {
        return Err(Layout::Signature);
    }
    let version = u32::from_le_bytes([file[8], file[9], file[10], file[11]]);
    let n = usize::from(u16::from_le_bytes([file[12], file[13]]));
    if file.len() < HEADER + 8 * n {
        return Err(Layout::Table);
    }
    let le16 = |at: usize| usize::from(u16::from_le_bytes([file[at], file[at + 1]]));
    let le32 = |at: usize| u32::from_le_bytes([file[at], file[at + 1], file[at + 2], file[at + 3]]);
    let want = usize::from(rom) + 1;
    let Some(i) = (0..n).find(|&i| le16(HEADER + 2 * i) == want) else {
        return Err(Layout::NoPatch);
    };
    let len = le16(HEADER + 2 * n + 2 * i);
    let at = le32(HEADER + 4 * n + 4 * i) as usize;
    // A patch inside the header or the tables, shorter than the version it
    // ends with, or running past the file, is not a patch.
    if at < HEADER + 8 * n || len < 4 || len > file.len().saturating_sub(at) {
        return Err(Layout::Bounds);
    }
    Ok(Patch { body: &file[at..at + len - 4], version })
}
