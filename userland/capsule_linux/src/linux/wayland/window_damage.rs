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

//! Where a guest window's new frame is on screen. Pure, so the host proofs
//! hold it.
//!
//! The compositor takes damage in screen coordinates. Every commit named
//! (0, 0) and the frame's size, the screen's top-left corner, while the
//! window sits centred: the compositor recomposed the corner, and of the
//! window only the part that overlapped it. The rest of a Qwen window kept
//! the pixels of its last full repaint, its new text did not show, and
//! whatever had been composed over it (another window's frame as it moved
//! away) stayed there.

/// The screen rectangle (`x`, `y`, `width`, `height`) a `width` by `height`
/// frame of a window placed at `at` covers; None while it is not placed.
pub fn window_damage(
    at: Option<(u32, u32)>,
    width: u32,
    height: u32,
) -> Option<(u32, u32, u32, u32)> {
    let (x, y) = at?;
    (width != 0 && height != 0).then_some((x, y, width, height))
}
