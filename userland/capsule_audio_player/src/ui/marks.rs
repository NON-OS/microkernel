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

//! The player's transport marks on the shared widgets: a glyph on a bare
//! button body, the round play button, and a slider laid over a groove so its
//! knob travels the groove's length, the line a click on it is read along.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::KNOB_R;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, Button, Kind, Slider, State};

use super::geometry::Rect;
use super::icon::{Glyph, Icons};

/// `g` at `size` in `ink`, centred on a bare button body that draws `look`.
pub fn glyph_on(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    (g, size, ink): (Glyph, i32, u32),
    look: State,
) {
    Button::new("").kind(Kind::Plain).state(look).draw(fb, r.tk());
    icons.centred(fb, r, size, g, ink);
}

/// The accent disc of radius `rad` centred in `r` with `g` on it: held, it
/// moves away from its glyph's ink, and hovered it takes the hover wash.
pub fn play_disc(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    (g, size, rad): (Glyph, i32, i32),
    look: State,
) {
    let (t, b) = (roles(), Rect::new(r.cx(), r.cy(), 0, 0).tk());
    let fill = if look.pressed { draw::pressed(t.accent, t.on_accent) } else { t.accent };
    draw::dot(fb, b.x, b.y, rad.max(0) as u32, fill);
    if look.hovered {
        draw::dot(fb, b.x, b.y, rad.max(0) as u32, t.hover_wash);
    }
    icons.centred(fb, r, size, g, t.on_accent.argb());
}

/// A slider at `value` of `max` whose knob travels the length of `groove`.
pub fn slider(fb: &mut PaintBuffer, groove: Rect, value: u32, max: u32, look: State) {
    let (knob, h) = (scale().px(KNOB_R) as i32, Slider::new(0, 0).size().1 as i32);
    let at = Rect::new(groove.x - knob, groove.cy() - h / 2, groove.w + knob * 2, h);
    Slider::new(value, max).state(look).draw(fb, at.tk());
}
