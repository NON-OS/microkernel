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

//! A colour as a cell records it.

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Color {
    /// The theme's foreground or background, whichever slot this is.
    #[default]
    Default,
    /// One of the 256 palette entries.
    Indexed(u8),
    Rgb(u8, u8, u8),
}

pub fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | b as u32
}

/// Halve a colour toward black, for dim text.
pub fn dim(c: u32) -> u32 {
    (c >> 1) & 0x7F7F7F
}
