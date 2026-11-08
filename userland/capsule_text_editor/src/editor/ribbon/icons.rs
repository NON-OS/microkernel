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

//! The pill chevron, painted from rectangles rather than a font glyph so it
//! stays crisp and needs no codepoint the bundled faces might not carry.

use nonos_app_skeleton::PaintBuffer;

pub(super) fn chevron(fb: &mut PaintBuffer, x: u32, cy: u32, argb: u32) {
    for i in 0..4u32 {
        fb.fill_rect(x + i, cy.saturating_sub(2) + i, 1, 1, argb);
        fb.fill_rect(x + 6 - i, cy.saturating_sub(2) + i, 1, 1, argb);
    }
}
