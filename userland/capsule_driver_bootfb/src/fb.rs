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

//! The framebuffer firmware left: a linear run of 32-bit pixels at a
//! fixed mode, laid out blue first (UEFI's PixelBlueGreenRedReserved8Bit
//! PerColor, and the usual simple-framebuffer a8r8g8b8 or x8r8g8b8) or
//! red first (PixelRedGreenBlueReserved8BitPerColor, x8b8g8r8). Other
//! layouts (16-bit, bit masks wider than a byte) are not served.

/// Which byte of a pixel holds blue.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout {
    /// Blue in byte 0: the wire's own ARGB8888 order, copied as it is.
    BlueFirst,
    /// Red in byte 0: red and blue swap on the way in.
    RedFirst,
}

/// A mapped framebuffer, checked once when the driver maps it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fb {
    pub base: *mut u8,
    pub len: usize,
    pub width: u32,
    pub height: u32,
    /// Bytes per line, at least four per pixel.
    pub stride: u32,
    pub layout: Layout,
    /// The panel's size as firmware's EDID gave it, w | h << 16 mm.
    pub physical_mm: u32,
}

impl Fb {
    /// None for a mapping too small for the mode, a stride shorter than a
    /// line, an empty mode, or a base not aligned to a pixel.
    pub fn new(
        base: *mut u8,
        len: usize,
        size: (u32, u32),
        stride: u32,
        layout: Layout,
    ) -> Option<Fb> {
        let (width, height) = size;
        let line = (width as u64).checked_mul(4)?;
        let need = (stride as u64).checked_mul(height as u64)?;
        let ok = width > 0 && height > 0 && stride as u64 >= line && need <= len as u64;
        if !ok || !(base as usize).is_multiple_of(4) {
            return None;
        }
        Some(Fb { base, len, width, height, stride, layout, physical_mm: 0 })
    }
}
