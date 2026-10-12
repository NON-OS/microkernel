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

//! MkBootLogo's which=0 answer, 28 bytes little-endian: x, y, width, height
//! u32; flags u8; quarter turns u8; bits per pixel u16; file size u32; the
//! pixel array's offset into the file u32 (the lead's F-50 note).

pub const RECORD_LEN: usize = 28;
/// The firmware had the logo on screen when it handed over.
pub const ON_SCREEN: u8 = 1 << 0;
/// The file's rows run top to bottom (a negative BMP height).
pub const TOP_DOWN: u8 = 1 << 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Record {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub flags: u8,
    /// Clockwise quarter turns the firmware showed the image at, 0 to 3.
    pub turns: u8,
    pub bpp: u16,
    pub file_size: u32,
    pub pixel_offset: u32,
}

impl Record {
    /// None for a short buffer or a turn count past three.
    pub fn parse(b: &[u8]) -> Option<Record> {
        let b: &[u8; RECORD_LEN] = b.get(..RECORD_LEN)?.try_into().ok()?;
        let u = |o: usize| u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
        let r = Record {
            x: u(0),
            y: u(4),
            width: u(8),
            height: u(12),
            flags: b[16],
            turns: b[17],
            bpp: u16::from_le_bytes([b[18], b[19]]),
            file_size: u(20),
            pixel_offset: u(24),
        };
        (r.turns < 4).then_some(r)
    }
}
