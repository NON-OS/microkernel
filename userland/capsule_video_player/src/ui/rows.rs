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

//! Where the list starts so the selected video stays in sight. Pure, so the
//! proofs drive it without a window.

use nonos_app_skeleton::scroll::{wheel_offset, WHEEL_LINES};

/// The first index to show after the selection moved to `sel`, given the
/// current first index `scroll`, how many entries fit (`visible`) and how
/// many share a line (`stride`: 1 in the list, the column count in the grid).
/// The result is always the start of a line, so the grid never reflows by
/// one tile; it moves only when `sel` would be off screen, and then by as
/// few lines as bring it back.
pub fn scroll_for(sel: usize, scroll: usize, visible: usize, stride: usize) -> usize {
    let stride = stride.max(1);
    let lines = (visible / stride).max(1);
    let line = sel / stride;
    let top = scroll / stride;
    let top = if line < top {
        line
    } else if line >= top + lines {
        line + 1 - lines
    } else {
        top
    };
    top * stride
}

/// The first index to show after a wheel `delta_y`, the selection left where
/// it is. A notch moves the grid one line of tiles and the list three rows, a
/// notch away toward the top; the start stays on a line, and the last line
/// stops at the bottom of the page rather than scrolling off it.
pub fn wheel_scroll(
    scroll: usize,
    len: usize,
    visible: usize,
    stride: usize,
    delta_y: i32,
) -> usize {
    let stride = stride.max(1);
    let shown = (visible / stride).max(1);
    let last = len.div_ceil(stride).saturating_sub(shown);
    let step = if stride > 1 { 1 } else { WHEEL_LINES };
    wheel_offset(scroll / stride, delta_y, step, last) * stride
}
