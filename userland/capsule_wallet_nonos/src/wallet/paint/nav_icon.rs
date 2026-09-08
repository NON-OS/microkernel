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

//! One glyph per navigation entry.
//!
//! Every entry in the rail used to draw the same twelve-pixel square, six of
//! them down the side of the window, differing only in colour when selected. A
//! row of identical squares is not iconography; it is a placeholder that shipped,
//! and it is the first thing that makes a window look unfinished.
//!
//! Drawn from rectangles, like the quick-action icons and for the same reason:
//! the bundled face has no arrow or shield codepoints, so glyphs set as text came
//! out as empty boxes. Shapes always render, at any size, on any face.

use nonos_app_skeleton::PaintBuffer;

use super::nav_glyph as glyph;

/// The box every glyph is drawn inside.
pub const BOX: u32 = 14;

#[derive(Clone, Copy)]
pub enum Nav {
    Home,
    Receive,
    Send,
    Proof,
    Shielded,
    Token,
}

pub fn icon(fb: &mut PaintBuffer, x: u32, y: u32, kind: Nav, c: u32) {
    match kind {
        Nav::Home => glyph::home(fb, x, y, c),
        Nav::Receive => glyph::arrow(fb, x, y, c, true),
        Nav::Send => glyph::arrow(fb, x, y, c, false),
        Nav::Proof => glyph::tick(fb, x, y, c),
        Nav::Shielded => glyph::shield(fb, x, y, c),
        Nav::Token => glyph::token(fb, x, y, c),
    }
}
