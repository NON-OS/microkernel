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

//! The `format` strings of a `simple-framebuffer` node (devicetree binding
//! `simple-framebuffer`), read as the bytes of one pixel in memory, lowest
//! address first. The binding names each format by its packed little-endian
//! word, most significant field first, so "x8r8g8b8" puts blue in the first
//! byte.

/// The pixel layouts the kernel can draw: 32-bit with one ignored byte, and
/// 24-bit packed. 16-bit and 10-bit formats are refused, not approximated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelFormat {
    /// Bytes R, G, B, then one ignored: "x8b8g8r8", "a8b8g8r8".
    Rgbx,
    /// Bytes B, G, R, then one ignored: "x8r8g8b8", "a8r8g8b8".
    Bgrx,
    /// Bytes B, G, R: "r8g8b8".
    Bgr,
}

impl PixelFormat {
    /// The format the string names, or `None` for one the kernel cannot
    /// draw. An alpha byte is treated as ignored: scanout does not blend.
    pub fn parse(format: &[u8]) -> Option<Self> {
        match format {
            b"x8b8g8r8" | b"a8b8g8r8" => Some(Self::Rgbx),
            b"x8r8g8b8" | b"a8r8g8b8" => Some(Self::Bgrx),
            b"r8g8b8" => Some(Self::Bgr),
            _ => None,
        }
    }

    pub const fn bytes_per_pixel(self) -> u32 {
        match self {
            Self::Rgbx | Self::Bgrx => 4,
            Self::Bgr => 3,
        }
    }
}
