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

//! Two small pieces the mark needs: its header, and blending.

/// Width (at 4) or height (at 6) from an NXM1 header.
pub fn dim(b: &[u8], at: usize) -> Option<u32> {
    if b.get(..4)? != b"NXM1" {
        return None;
    }
    Some(u16::from_le_bytes([*b.get(at)?, *b.get(at + 1)?]) as u32)
}

/// `fg` over `bg` at `a` of 255.
pub fn mix(bg: u32, fg: u32, a: u32) -> u32 {
    let ch = |s: u32| (((bg >> s) & 0xFF) * (255 - a) + ((fg >> s) & 0xFF) * a) / 255;
    0xFF00_0000 | (ch(16) << 16) | (ch(8) << 8) | ch(0)
}
