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

//! A track's tile on Home. The shared Card is the body, and draws hover and
//! press; on it sit the cover with the track's format as a Pill, the title
//! over the artist, and the play mark the playing track's tile carries.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::derive::with_alpha;
use nonos_toolkit::theme::{roles, Opaque};
use nonos_toolkit::widgets::{draw, text, Card, Pill, State, Tone};

use crate::library::Track;
use crate::ui::art::{cover_scrimmed, tint};
use crate::ui::geometry::Rect;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::metrics::{line_h, metrics, px};

pub fn tile(fb: &mut PaintBuffer, icons: &Icons, r: Rect, t: &Track, playing: bool, look: State) {
    let (th, m) = (roles(), metrics());
    let (s2, s3, item, secondary) = (m.s2, m.s3, m.item, m.secondary);
    Card::new().state(look).draw(fb, r.tk());
    let room = r.h - s2 - s3 - line_h(item) - line_h(secondary);
    let side = r.w - s2 * 2;
    let art = Rect::new(r.x + s2, r.y + s2, side, side.min(room).max(0));
    cover_scrimmed(fb, art, &t.title, m.r_cover);
    let pill = Pill::new(&t.format).tone(Tone::Accent);
    pill.draw(fb, Rect::new(art.x + s3, art.y + s3, art.w - s3 * 2, pill.size().1 as i32).tk());
    if playing {
        let s = (art.w / 4).max(px(20));
        let c = Rect::new(art.right() - s - s3, art.bottom() - s - s3, s, s);
        let (b, accent) = (c.tk(), tint(&t.title));
        let glow = with_alpha(accent, 0x55);
        fb.shadow_round(b.x, b.y, b.w, b.h, (s / 2) as u32, (s / 3) as u32, glow);
        draw::dot(fb, b.x + b.w / 2, b.y + b.h / 2, (s / 2) as u32, Opaque::new(accent));
        icons.centred(fb, c, s / 2, Glyph::Play, th.on_accent.argb());
    }
    let (tx, tw, ty) = ((r.x + s3).max(0) as u32, (r.w - s3 * 2).max(0) as u32, art.bottom() + s3);
    text::left(fb, tx, ty, &text::truncate(&t.title, tw, item), th.text, item);
    let sub = text::truncate(&t.artist, tw, secondary);
    text::left(fb, tx, ty + line_h(item) - s2, &sub, th.text_2, secondary);
}
