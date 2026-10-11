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

//! The 18 byte detailed timing descriptor, E-EDID 1.4 section 3.10.2,
//! Table 3.21. Each field is a low byte plus high bits packed elsewhere.

use super::dtd::Detailed;

/// The descriptor's length, in the base block and in CTA-861 extensions.
pub const LEN: usize = 18;

impl Detailed {
    /// Decode one descriptor. None when it is a display descriptor
    /// (bytes 0-1, the pixel clock, are zero) or names no picture.
    pub fn decode(d: &[u8]) -> Option<Self> {
        let d = d.get(..LEN)?;
        // Bytes 0-1: the pixel clock in 10 kHz units, little-endian.
        let clock = u16::from_le_bytes([d[0], d[1]]) as u32;
        if clock == 0 {
            return None;
        }
        let lo = |i: usize| d[i] as u32;
        let hi = |i: usize, shift: u32, mask: u32| (d[i] as u32 >> shift) & mask;
        let flags = d[17];
        // Byte 17 bits 4-3: 11 digital separate sync carries both
        // polarities (bits 2 and 1); 10 digital composite only the
        // horizontal one in bit 1; analog sync carries neither.
        let sync = (flags >> 3) & 3;
        let t = Detailed {
            pixel_khz: clock * 10,
            h_active: lo(2) | hi(4, 4, 0xF) << 8,
            h_blank: lo(3) | hi(4, 0, 0xF) << 8,
            v_active: lo(5) | hi(7, 4, 0xF) << 8,
            v_blank: lo(6) | hi(7, 0, 0xF) << 8,
            h_front: lo(8) | hi(11, 6, 3) << 8,
            h_sync: lo(9) | hi(11, 4, 3) << 8,
            v_front: hi(10, 4, 0xF) | hi(11, 2, 3) << 4,
            v_sync: hi(10, 0, 0xF) | hi(11, 0, 3) << 4,
            h_positive: sync >= 2 && flags & 0x02 != 0,
            v_positive: sync == 3 && flags & 0x04 != 0,
            interlaced: flags & 0x80 != 0,
            width_mm: lo(12) | hi(14, 4, 0xF) << 8,
            height_mm: lo(13) | hi(14, 0, 0xF) << 8,
        };
        if t.h_active == 0 || t.v_active == 0 {
            return None;
        }
        Some(t)
    }
}
