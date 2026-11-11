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

use nonos_app_skeleton::paint::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::KNOB_R;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, text, Kind, Rect, Slider, State};

use super::transport::{transport, Transport};
use crate::ui::chrome::BarState;
use crate::ui::hover::Lit;
use crate::ui::icon;
use crate::ui::layout::Layout;
use crate::ui::marks::glyph_on;
use crate::ui::target::Target;
use crate::ui::text::{body_px, center_y, hhmmss};

const GLYPH: u32 = 18;
const NO_SOUND: &str = "Picture only, no sound";

/// Play or pause on an accent body, drawn in the box its click answers in.
fn paint_play(fb: &mut PaintBuffer, t: &Transport, playing: bool, look: State) {
    let glyph = if playing { icon::transport::pause } else { icon::transport::play };
    let size = scale().px(GLYPH);
    glyph_on(fb, t.play, (glyph, size, roles().on_accent), Kind::Primary, look);
}

fn paint_sides(fb: &mut PaintBuffer, t: &Transport, lit: &Lit) {
    let theme = roles();
    let (ink, size) = (theme.text, scale().px(GLYPH));
    glyph_on(fb, t.prev, (icon::transport::prev, size, ink), Kind::Plain, lit.of(Target::Prev));
    glyph_on(fb, t.next, (icon::transport::next, size, ink), Kind::Plain, lit.of(Target::Next));
    /* Said in place of a volume control: the stream has no sound to set. */
    let w = text::width(NO_SOUND, body_px());
    if w <= t.note.w {
        let y = center_y(t.note.y, t.note.h);
        text::right(fb, t.note.x + t.note.w, y, NO_SOUND, theme.muted, body_px());
    }
}

fn paint_times(fb: &mut PaintBuffer, l: &Layout, st: &BarState) {
    let mut a = [0u8; 8];
    let mut b = [0u8; 8];
    let s = scale();
    let (y, theme) = (center_y(l.scrub.y + s.px(14), s.px(20)), roles());
    text::left(fb, l.elapsed_x, y, hhmmss(st.elapsed_ms, &mut a), theme.text, body_px());
    text::right(fb, l.remain_x, y, hhmmss(st.total_ms, &mut b), theme.text_2, body_px());
}

/// The seek groove as a slider whose knob travels the groove's length, the
/// line a click on it is read along.
fn paint_seek(fb: &mut PaintBuffer, l: &Layout, permille: u32, look: State) {
    let (knob, h) = (scale().px(KNOB_R), Slider::new(0, 0).size().1);
    let (g, cy) = (l.scrub, l.scrub.y + l.scrub.h / 2);
    let (x, y) = (g.x.saturating_sub(knob), cy.saturating_sub(h / 2));
    let at = Rect { x, y, w: g.w + knob * 2, h };
    Slider::new(permille, 1000).state(look).draw(fb, at);
}

pub fn paint_transport(fb: &mut PaintBuffer, l: &Layout, st: &BarState, lit: &Lit) {
    draw::fill(fb, l.bar, roles().surface);
    paint_seek(fb, l, st.permille, lit.of(Target::Seek));
    paint_times(fb, l, st);
    let t = transport(l, fb.width);
    paint_play(fb, &t, st.playing, lit.of(Target::Play));
    paint_sides(fb, &t, lit);
}
