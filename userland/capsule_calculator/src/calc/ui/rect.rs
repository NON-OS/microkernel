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

//! The calculator lays out in signed pixels; this turns a box it worked out
//! into the toolkit's `Rect`, which the widgets draw in and the hit tests read.

use nonos_toolkit::widgets::Rect;

/// The box at `(x, y)`, `w` by `h`. A window too small for the box gives a
/// negative size, which becomes an empty box that draws and names nothing.
pub fn rect(x: i32, y: i32, w: i32, h: i32) -> Rect {
    let u = |v: i32| v.max(0) as u32;
    Rect { x: u(x), y: u(y), w: u(w), h: u(h) }
}
