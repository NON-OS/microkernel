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

//! The transport bar across the foot of the window: cover thumb and metadata
//! on the left, the play cluster at the centre, scrubber and volume on the
//! right.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, text, Button, Kind, Separator, State};

use crate::model::PlayerView;
use crate::transport::State as Play;
use crate::ui::art::cover;
use crate::ui::fmt::mmss;
use crate::ui::geometry::Rect;
use crate::ui::hover::Lit;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::marks::{glyph_on, play_disc, slider};
use crate::ui::metrics::{hair, metrics, px};
use crate::ui::target::{Key, Target};

use super::bar::{bar, play_r, Bar};

fn toggle(fb: &mut PaintBuffer, icons: &Icons, r: Rect, g: Glyph, on: bool, look: State) {
    let (t, kind) = (roles(), if on { Kind::Tonal } else { Kind::Plain });
    Button::new("").kind(kind).state(look).draw(fb, r.tk());
    icons.centred(fb, r, px(18), g, if on { t.accent } else { t.text_2 }.argb());
}

fn meta(fb: &mut PaintBuffer, b: &Bar, v: &PlayerView, id: &str) {
    let (t, m) = (roles(), metrics());
    cover(fb, b.thumb, id, m.r_thumb);
    draw::outline(fb, b.thumb.tk(), m.r_thumb, hair() as u32, t.border);
    let tx = b.thumb.right() + m.s4;
    let tw = (b.shuffle.x - m.s5 - tx).max(0) as u32;
    let named = if v.title.is_empty() { "Nothing playing" } else { v.title.as_str() };
    let title = text::truncate(named, tw, m.item);
    text::left(fb, tx.max(0) as u32, b.thumb.y + m.s2, &title, t.text, m.item);
    let (line, ink) = match v.notice {
        Some(notice) => (notice, t.danger),
        None => (v.artist.as_str(), t.muted),
    };
    let sub = text::truncate(line, tw, m.label);
    text::left(fb, tx.max(0) as u32, b.thumb.y + m.s2 + px(25), &sub, ink, m.label);
}

fn cluster(fb: &mut PaintBuffer, icons: &Icons, b: &Bar, v: &PlayerView, lit: &Lit) {
    let (t, at) = (roles(), |k| lit.of(Target::Bar(k)));
    toggle(fb, icons, b.shuffle, Glyph::Shuffle, v.shuffle, at(Key::Shuffle));
    glyph_on(fb, icons, b.prev, (Glyph::Prev, px(20), t.text.argb()), at(Key::Prev));
    glyph_on(fb, icons, b.next, (Glyph::Next, px(20), t.text.argb()), at(Key::Next));
    toggle(fb, icons, b.repeat, Glyph::Repeat, v.repeat, at(Key::Repeat));
    let g = if v.state == Play::Playing { Glyph::Pause } else { Glyph::Play };
    play_disc(fb, icons, b.play, (g, px(20), play_r()), at(Key::Play));
    let c = b.play.tk();
    let (cx, cy) = (c.x + c.w / 2, c.y + c.h / 2);
    fb.ring(cx, cy, (play_r() + px(4)) as u32, hair() as u32, t.accent_edge.argb());
}

fn scrubber(fb: &mut PaintBuffer, b: &Bar, r: Rect, v: &PlayerView, look: State) {
    let m = metrics();
    let stamps = Rect::new(b.scrub.x, r.y + m.s3, b.scrub.w, px(20)).tk();
    let (muted, top) = (roles().muted, text::centre_top(stamps.y, stamps.h, m.label));
    text::left(fb, stamps.x, top, &mmss(v.pos_ms), muted, m.label);
    text::right(fb, stamps.x + stamps.w, top, &mmss(v.dur_ms), muted, m.label);
    slider(fb, b.scrub, v.pos_ms, v.dur_ms.max(1), look);
}

pub fn transport(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    v: &PlayerView,
    id: &str,
    lit: &Lit,
) {
    let (t, at) = (roles(), |k| lit.of(Target::Bar(k)));
    draw::fill(fb, r.tk(), t.surface);
    Separator::new().draw(fb, Rect::new(r.x, r.y, r.w, hair()).tk());
    let b = bar(r);
    meta(fb, &b, v, id);
    cluster(fb, icons, &b, v, lit);
    scrubber(fb, &b, r, v, at(Key::Seek));
    let ink = if v.muted { t.danger } else { t.text_2 }.argb();
    glyph_on(fb, icons, b.speaker, (Glyph::Speaker, px(17), ink), at(Key::Mute));
    let vol = if v.muted { 0 } else { v.volume_q15.max(0) as u32 };
    slider(fb, b.volume, vol, 32768, at(Key::Volume));
}
