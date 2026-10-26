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
//! The pointer's shapes: the arrow, and the four a window's edges show
//! while a press there would resize it. Each is drawn from its hot spot,
//! the pixel at the pointer's position: the arrow's tip, a resize arrow's
//! middle. 'X' is the dark outline, 'o' the light fill.

use super::cursor::ARROW;
use super::cursor_diagonals::{NESW, NWSE};
use super::cursor_straights::{EW, NS};

pub struct Shape {
    pub rows: &'static [&'static [u8]],
    pub hot: (u32, u32),
}

/// The codes a window asks for its shape by (OP_CURSOR_SHAPE).
pub const ARROW_CODE: u8 = 0;
pub const EW_CODE: u8 = 1;
pub const NS_CODE: u8 = 2;
pub const NWSE_CODE: u8 = 3;
pub const NESW_CODE: u8 = 4;

pub const ARROW_SHAPE: Shape = Shape { rows: &ARROW, hot: (0, 0) };
/// The shape for `code`; the arrow for any code not one of them.
pub fn shape(code: u8) -> &'static Shape {
    match code {
        EW_CODE => &EW,
        NS_CODE => &NS,
        NWSE_CODE => &NWSE,
        NESW_CODE => &NESW,
        _ => &ARROW_SHAPE,
    }
}
