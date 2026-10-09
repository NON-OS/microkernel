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

//! Attribute bits on a cell.

pub const BOLD: u16 = 1 << 0;
pub const DIM: u16 = 1 << 1;
pub const ITALIC: u16 = 1 << 2;
/// Three bits holding an `Underline` style.
pub const UNDERLINE_SHIFT: u16 = 3;
pub const UNDERLINE_MASK: u16 = 0b111 << UNDERLINE_SHIFT;
pub const BLINK: u16 = 1 << 6;
pub const INVERSE: u16 = 1 << 7;
pub const HIDDEN: u16 = 1 << 8;
pub const STRIKE: u16 = 1 << 9;
pub const OVERLINE: u16 = 1 << 10;
/// The left half of a character two columns wide.
pub const WIDE: u16 = 1 << 11;
/// The right half. It draws nothing; the cell before it drew both halves.
pub const WIDE_TAIL: u16 = 1 << 12;
/// The attributes a pen carries. Width belongs to the character written.
pub const PEN: u16 = !(WIDE | WIDE_TAIL);
