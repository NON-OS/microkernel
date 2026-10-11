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

//! What a universal plane holds, read back (SKL PRM Vol 2c, PLANE_CTL,
//! PLANE_STRIDE, PLANE_SIZE, PLANE_SURF, PLANE_SURFLIVE).

use super::access::{field, Mmio};
use super::pipe::Pipe;
use super::plane_regs as r;

/// PLANE_CTL fields this driver reads or changes.
pub const ENABLE: u32 = 1 << 31;
/// Source Pixel Format, 27:24; 0100b is RGB 8:8:8:8.
pub const FORMAT_MASK: u32 = 0xF << 24;
pub const FORMAT_RGB8888: u32 = 0x4 << 24;
/// RGB Color Order: 0 keeps B in the lowest byte, ARGB8888's order
/// (bit placement unverified against the PRM).
pub const ORDER_RGBX: u32 = 1 << 20;
/// Tiled Surface, 12:10; 000b is linear.
pub const TILING_MASK: u32 = 0x7 << 10;
/// Plane Rotation, 1:0; 00b is none.
pub const ROTATION_MASK: u32 = 0x3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlaneState {
    pub ctl: u32,
    /// Bytes per line: PLANE_STRIDE counts 64 byte units for linear.
    pub stride: u32,
    pub width: u32,
    pub height: u32,
    /// The armed surface address and the one being scanned out.
    pub surf: u32,
    pub live: u32,
}

impl PlaneState {
    pub fn read(m: &mut dyn Mmio, p: Pipe) -> PlaneState {
        let size = m.read(r::plane_size(p, 1));
        PlaneState {
            ctl: m.read(r::plane_ctl(p, 1)),
            stride: field(m.read(r::plane_stride(p, 1)), 9, 0) * 64,
            width: field(size, 12, 0) + 1,
            height: field(size, 27, 16) + 1,
            surf: m.read(r::plane_surf(p, 1)) & !0xFFF,
            live: m.read(r::plane_surflive(p, 1)) & !0xFFF,
        }
    }

    pub fn enabled(&self) -> bool {
        self.ctl & ENABLE != 0
    }

    /// Linear, unrotated 32-bit RGB: a layout this driver can flip.
    pub fn flippable(&self) -> bool {
        let rgb = self.ctl & FORMAT_MASK == FORMAT_RGB8888;
        rgb && self.ctl & (TILING_MASK | ROTATION_MASK) == 0
    }
}
