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

use nonos_app_skeleton::{measure_ttf, PaintBuffer};
use nonos_libc::mk_time_millis;
use nonos_toolkit::design::scale::scale;

use crate::browser::omnibox::Focus;
use crate::browser::paint::glyph::{self, Glyph};
use crate::browser::paint::{clip, field_text, tokens::palette, FieldInk};
use crate::browser::shell::clock::hour_greeting;
use crate::browser::state::State;
use crate::browser::ui::start_geom::{pill_text, StartGeom};

const GREET_PX: f32 = 44.0;
const SUB_PX: f32 = 18.0;
const PILL_RADIUS: u32 = 26;
const RING: u32 = 2;
const SPREAD: u32 = 6;
const ICON: u32 = 20;
const ICON_X: u32 = 28;
const SUBLINE: &str = "Private by default.";
const HINT: &str = "Search or enter address";

fn ink() -> FieldInk {
    let p = palette();
    FieldInk { text: p.ink, hint: p.muted, select: p.accent_soft }
}

/// The sky band with the greeting, the search pill and the session badge.
pub fn paint(state: &State, g: &StartGeom, fb: &mut PaintBuffer) {
    clip::gradient_v(fb, g.hero, palette().sky, palette().ground);
    let (greet, sub) = (scale().font(GREET_PX), scale().font(SUB_PX));
    let greet_right = centred(fb, hour_greeting(mk_time_millis()), g.greet_y, palette().ink, greet);
    centred(fb, SUBLINE, g.sub_y, palette().muted, sub);
    pill(state, g, fb);
    super::badge::paint(fb, g.hero, greet_right);
}

fn centred(fb: &mut PaintBuffer, s: &str, y: i32, argb: u32, px: f32) -> i32 {
    let x = (fb.width as i32 - measure_ttf(s, px)) / 2;
    fb.text_ttf(x, y, s, argb, px)
}

/// The address bar's twin: the same text, caret and selection, drawn in a
/// wide pill on the sky. Keystrokes repaint the whole start page with it.
fn pill(state: &State, g: &StartGeom, fb: &mut PaintBuffer) {
    let (p, Some(t)) = (g.pill, pill_text(g)) else { return };
    let (x, y, w, h) = (p.x as u32, p.y as u32, p.w as u32, p.h as u32);
    let (focused, s) = (state.ui.kbd == Focus::Omnibox, scale());
    let radius = s.px(PILL_RADIUS);
    fb.shadow_round(x, y, w, h, radius, s.px(SPREAD), palette().shadow);
    fb.fill_round(x, y, w, h, radius, palette().surface);
    if focused {
        fb.stroke_round(x, y, w, h, radius, s.px(RING), palette().accent);
    }
    let (ix, icon) = (p.x + s.px(ICON_X) as i32, s.px(ICON) as i32);
    glyph::draw(fb, Glyph::Search, ix, p.y + p.h / 2, icon, palette().muted);
    let at = (focused, state.start_text_off(t.w));
    let boxed = (t.x as u32, t.y as u32, t.w as u32, t.h as u32);
    field_text(fb, boxed, &state.ui.omnibox, at, HINT, ink());
}
