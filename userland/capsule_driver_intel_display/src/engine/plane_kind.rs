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

//! The primary plane's control and stride fields per layout. Only the
//! fields this driver reads or changes are named; every other CTL bit
//! (gamma, CSC, keying) is firmware's and kept.

use super::gen::PlaneKind;

impl PlaneKind {
    /// The Source Pixel Format field.
    pub fn format_mask(self) -> u32 {
        match self {
            PlaneKind::Primary => 0xF << 26,
            PlaneKind::Universal => 0xF << 24,
            PlaneKind::Universal11 => 0x1F << 23,
        }
    }

    /// 32-bit RGB with blue in the lowest byte, ARGB8888's order: DSPCNTR
    /// 0110b "BGRX 8:8:8:8"; PLANE_CTL 0100b (Gen9) or 01000b (Gen11+),
    /// the same bits, with RGB Color Order left at 0.
    pub fn format_argb(self) -> u32 {
        match self {
            PlaneKind::Primary => 0x6 << 26,
            _ => 0x4 << 24,
        }
    }

    /// RGB Color Order on universal planes; DSPCNTR folds order into the
    /// format, so there is no such bit there.
    pub fn order_bit(self) -> u32 {
        match self {
            PlaneKind::Primary => 0,
            _ => 1 << 20,
        }
    }

    /// Tiling and rotation: DSPCNTR bit 10 (X tiled) and bit 15 (180
    /// degrees); PLANE_CTL 12:10 and 1:0.
    pub fn layout_mask(self) -> u32 {
        match self {
            PlaneKind::Primary => (1 << 10) | (1 << 15),
            _ => (0x7 << 10) | 0x3,
        }
    }
}
