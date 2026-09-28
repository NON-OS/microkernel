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

//! Drawing the terminal body from the screen and its history.

mod area;
mod body;
mod box_arms;
mod box_draw;
mod cell;
mod cursor;
mod deco;
mod fill;
mod find_bar;
mod rows;

pub use area::{Area, Frame, Shade};
pub use body::draw_vt;
pub use find_bar::draw_find_bar;
pub use rows::Rows;
