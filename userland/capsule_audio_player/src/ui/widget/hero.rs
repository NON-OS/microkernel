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


//! The Home banner: an eyebrow, the feature title, one line of copy, a pair of
//! actions and the stamp block pinned to the top right, over a cover wash that
//! bleeds in from the right edge.

extern crate alloc;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, text, Kind, State};

use crate::ui::art::cover;
use crate::ui::drawn::labelled;
use crate::ui::geometry::Rect;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::metrics::{cap_h, hair, line_h, metrics, px};
use crate::ui::screen::home_geom::action_rect;

pub fn hero(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    eyebrow: &str,
    title: &str,
    copy: &str,
    stamp: (&str, &str),
    actions: [(&str, Glyph); 2],
    looks: [State; 2],
) {
    let (t, m) = (roles(), metrics());
    let (ink, accent, rad, label) = (t.text, t.accent, m.r_card, m.label);
    draw::round(fb, r.tk(), rad, t.raised);
    let wash = Rect::new(r.x + r.w * 42 / 100, r.y, r.w - r.w * 42 / 100, r.h);
    cover(fb, wash, title, rad);
    draw::round(fb, Rect::new(r.x, r.y, r.w * 62 / 100, r.h).tk(), rad, t.raised);
    draw::outline(fb, r.tk(), rad, hair() as u32, t.border);

    let x = r.x.max(0) as u32 + m.s7 as u32;
    let tw = (r.w * 58 / 100 - m.s7).max(0) as u32;
    let mut y = r.y + m.s6;
    text::left(fb, x, y, &eyebrow.to_ascii_uppercase(), accent, label);
    y += line_h(label) + m.s3;
    text::left(fb, x, y, &text::truncate(title, tw, m.hero), ink, m.hero);
    y += line_h(m.hero) - m.s3;
    text::left(fb, x, y, &text::truncate(copy, tw, m.body), t.text_2, m.body);

    let head = Rect::new(r.x + r.w / 2, r.y + m.s6, r.w / 2 - m.s7, cap_h(label) * 2);
    let right = |b: Rect| (b.right().max(0) as u32, text::centre_top(b.tk().y, b.tk().h, label));
    let (hx, hy) = right(head);
    text::right(fb, hx, hy, stamp.0, ink, label);
    let sub = Rect::new(head.x, head.bottom() + m.s4, head.w, cap_h(label) * 2);
    let (sx, sy) = right(sub);
    text::right(fb, sx, sy, stamp.1, t.muted, label);
    let tick = Rect::new(sub.right() - px(38), sub.bottom() + m.s4, px(38), hair());
    draw::fill(fb, tick.tk(), accent);

    let [(first, g0), (second, g1)] = actions;
    labelled(fb, icons, action_rect(r, 0), first, Some(g0), Kind::Primary, looks[0]);
    labelled(fb, icons, action_rect(r, 1), second, Some(g1), Kind::Secondary, looks[1]);
}
