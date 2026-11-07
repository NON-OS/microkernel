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

//! wp_cursor_shape_v1's shapes as the compositor's: the arrow and the four
//! resize arrows it draws (compositor frame_pacer/cursor_shapes.rs). A shape
//! it has no drawing of is the arrow. Pure, so the host proofs hold it.

pub const ARROW: u8 = 0;
pub const EW: u8 = 1;
pub const NS: u8 = 2;
pub const NWSE: u8 = 3;
pub const NESW: u8 = 4;

/// The compositor's code for wp_cursor_shape_device_v1 shape `shape`.
pub fn code_of(shape: u32) -> u8 {
    match shape {
        // e, w, ew and col resize.
        18 | 25 | 26 | 30 => EW,
        // n, s, ns and row resize.
        19 | 22 | 27 | 31 => NS,
        // nw, se and nwse resize.
        21 | 23 | 29 => NWSE,
        // ne, sw and nesw resize.
        20 | 24 | 28 => NESW,
        _ => ARROW,
    }
}
