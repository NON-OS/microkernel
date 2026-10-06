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

//! Where the wheel takes the listing and an opened file, on the shared wheel
//! rule: a notch away from the user moves toward the top, and neither view
//! scrolls past its content.

use nonos_app_skeleton::scroll::{wheel_offset, WHEEL_LINES};

/// The listing's first entry after a wheel `delta_y`. `visible` is how many
/// entries the view shows and `cols` how many share a line: 1 in the list,
/// the column count in the icon grid. A notch moves the list three rows and
/// the grid one line of icons, the start stays on a line, and the last line
/// stops at the bottom of the view.
pub fn listing(scroll: usize, len: usize, visible: usize, cols: usize, delta_y: i32) -> usize {
    let cols = cols.max(1);
    let shown = (visible / cols).max(1);
    let last = len.div_ceil(cols).saturating_sub(shown);
    let step = if cols > 1 { 1 } else { WHEEL_LINES };
    wheel_offset(scroll / cols, delta_y, step, last) * cols
}

/// An opened file's first line after a wheel `delta_y`, three lines a notch,
/// with `visible` lines on screen.
pub fn preview(scroll: usize, lines: usize, visible: usize, delta_y: i32) -> usize {
    wheel_offset(scroll, delta_y, WHEEL_LINES, lines.saturating_sub(visible))
}
