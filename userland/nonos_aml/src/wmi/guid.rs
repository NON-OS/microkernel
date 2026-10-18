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

//! A GUID as text, the way the documents that publish WMI GUIDs write it:
//! the first three fields little-endian as stored, the last two as bytes.

use alloc::string::String;
use core::fmt::Write;

pub fn guid_text(g: &[u8; 16]) -> String {
    let d1 = u32::from_le_bytes([g[0], g[1], g[2], g[3]]);
    let d2 = u16::from_le_bytes([g[4], g[5]]);
    let d3 = u16::from_le_bytes([g[6], g[7]]);
    let mut s = String::with_capacity(36);
    let _ = write!(s, "{d1:08X}-{d2:04X}-{d3:04X}-{:02X}{:02X}-", g[8], g[9]);
    for b in &g[10..] {
        let _ = write!(s, "{b:02X}");
    }
    s
}
