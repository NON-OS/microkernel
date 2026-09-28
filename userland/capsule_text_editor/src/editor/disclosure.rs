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

/*
 * The chevron beside a folder, drawn in pixels: a font without the
 * triangle characters draws them as empty boxes, and a shape this small
 * reads better on the pixel grid than antialiased at twelve points anyway.
 */

use nonos_app_skeleton::PaintBuffer;

/// Half the triangle's long side, in pixels.
const HALF: u32 = 4;

/// A chevron centred on `(cx, cy)`: pointing down when open, right when not.
pub(super) fn disclosure(fb: &mut PaintBuffer, cx: u32, cy: u32, open: bool, argb: u32) {
    for i in 0..HALF {
        let span = 2 * (HALF - i) - 1;
        if open {
            fb.fill_rect(cx + 1 + i - HALF, cy + i - HALF / 2, span, 1, argb);
        } else {
            fb.fill_rect(cx + i - HALF / 2, cy + 1 + i - HALF, 1, span, argb);
        }
    }
}
