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

//! The two diagonals, stepped a pixel at a time.
//!
//! Drawn from rects because the toolkit has no vector path, and a font glyph
//! would sit on the text baseline rather than on the row's own centre. An
//! anti-aliased curve would blur to a smudge at eleven pixels across.

use nonos_app_skeleton::PaintBuffer;

use super::verify_mark::{MARK, STROKE};

// A tick as a short arm and a long one, stepped a pixel at a time so both read
// as diagonals at this size; an anti-aliased curve would blur to a smudge in a
// glyph 11 pixels across.
pub(super) fn tick(fb: &mut PaintBuffer, x: u32, cy: u32, argb: u32) {
    let foot = MARK / 3;
    for i in 0..foot {
        fb.fill_rect(x + 1 + i, cy.saturating_sub(1) + i, STROKE, STROKE, argb);
    }
    for i in 0..MARK.saturating_sub(foot) {
        let step = foot.saturating_sub(1) + i;
        let up = (foot + i).saturating_sub(1);
        fb.fill_rect(x + 1 + step, (cy + 1).saturating_sub(up), STROKE, STROKE, argb);
    }
}

pub(super) fn cross(fb: &mut PaintBuffer, x: u32, cy: u32, argb: u32) {
    let arm = MARK.saturating_sub(3);
    let top = cy.saturating_sub(arm / 2);
    for i in 0..arm {
        fb.fill_rect(x + 2 + i, top + i, STROKE, STROKE, argb);
        fb.fill_rect(x + 2 + i, top + arm.saturating_sub(1 + i), STROKE, STROKE, argb);
    }
}
