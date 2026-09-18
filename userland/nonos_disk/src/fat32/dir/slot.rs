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

//! One 32-byte directory slot. Timestamps are a fixed date rather than a
//! clock reading, so two installs of the same image produce the same
//! sectors and a disk can be checked against a reference by hashing it.

pub const ATTR_DIR: u8 = 0x10;
pub const ATTR_ARCHIVE: u8 = 0x20;

/// 2026-01-01, the same fixed stamp the USB image build touches its files
/// to, in FAT date encoding: (year - 1980) << 9 | month << 5 | day.
const DATE: u16 = (46 << 9) | (1 << 5) | 1;

pub fn slot(name: [u8; 11], attr: u8, nt_flags: u8, cluster: u32, size: u32) -> [u8; 32] {
    let mut s = [0u8; 32];
    s[0..11].copy_from_slice(&name);
    s[11] = attr;
    s[12] = nt_flags;
    s[16..18].copy_from_slice(&DATE.to_le_bytes());
    s[18..20].copy_from_slice(&DATE.to_le_bytes());
    s[20..22].copy_from_slice(&((cluster >> 16) as u16).to_le_bytes());
    s[24..26].copy_from_slice(&DATE.to_le_bytes());
    s[26..28].copy_from_slice(&(cluster as u16).to_le_bytes());
    s[28..32].copy_from_slice(&size.to_le_bytes());
    s
}
