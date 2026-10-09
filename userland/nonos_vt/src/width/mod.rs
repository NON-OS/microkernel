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

//! How many columns a character takes: none for marks that combine with the
//! character before them, two for East Asian wide and emoji presentation,
//! else one. The tables follow Unicode 15 East Asian Width and general
//! category Mn, the same split wcwidth makes, so a program's idea of a
//! line's width and the screen's agree.

#[cfg(test)]
mod check;
mod lookup;
mod wide;
mod zero_high;
mod zero_low;

pub use lookup::{is_emoji_modifier, width, ZWJ};
