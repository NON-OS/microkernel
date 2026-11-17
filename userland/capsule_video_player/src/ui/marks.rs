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

//! The player's own glyphs on the shared Button body. The shared icon set has
//! no play, pause, skip, back, info, grid or list mark, so the body is the
//! toolkit's and the glyph is drawn on it here.

use nonos_app_skeleton::paint::PaintBuffer;
use nonos_toolkit::theme::Opaque;
use nonos_toolkit::widgets::rect::centre;
use nonos_toolkit::widgets::{Button, Kind, Rect, State};

/// One of the player's glyphs: drawn at `x`, `y` as a square `size` across in
/// an ARGB ink.
pub type Glyph = fn(&mut PaintBuffer, u32, u32, u32, u32);

/// `glyph` at `size` in `ink`, centred on a bare `kind` button body in `r`
/// that draws `look`.
pub fn glyph_on(
    fb: &mut PaintBuffer,
    r: Rect,
    (glyph, size, ink): (Glyph, u32, Opaque),
    kind: Kind,
    look: State,
) {
    Button::new("").kind(kind).state(look).draw(fb, r);
    let g = centre(r, size, size);
    glyph(fb, g.x, g.y, g.w.min(g.h), ink.argb());
}
