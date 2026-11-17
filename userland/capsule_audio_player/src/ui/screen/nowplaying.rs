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

//! Now Playing: the cover at display size, the metadata block beside it, and
//! the decoded waveform under both. The transport controls stay in the shell
//! bar rather than being duplicated here.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, text, Pill};

use crate::model::PlayerView;
use crate::ui::art::cover;
use crate::ui::fmt::mmss;
use crate::ui::geometry::Rect;
use crate::ui::metrics::{hair, line_h, metrics, px};
use crate::ui::widget::waveform;
use crate::waveform::Waveform;

const ART: i32 = 300;

fn art_side(r: Rect) -> i32 {
    (r.w * 42 / 100).clamp(px(180), px(ART))
}

pub fn art_rect(r: Rect) -> Rect {
    let s = art_side(r);
    Rect::new(r.x, r.y + metrics().s4, s, s)
}

pub fn wave_rect(r: Rect) -> Rect {
    let (a, m) = (art_rect(r), metrics());
    let h = (r.bottom() - a.bottom() - m.s6 - m.s4).clamp(0, px(120));
    Rect::new(r.x, a.bottom() + m.s6, r.w, h)
}

fn stat(fb: &mut PaintBuffer, r: Rect, label: &str, value: &str) {
    let (t, x, m) = (roles(), r.x.max(0) as u32, metrics());
    text::left(fb, x, r.y, &label.to_ascii_uppercase(), t.muted, m.label);
    text::left(fb, x, r.y + line_h(m.label), value, t.text, m.hero);
}

pub fn paint(fb: &mut PaintBuffer, r: Rect, v: &PlayerView, w: &Waveform, id: &str) {
    let (a, t, m) = (art_rect(r), roles(), metrics());
    let (rad, label, display) = (m.r_card, m.label, m.display);
    draw::round(fb, a.inset(-m.s3).tk(), rad + px(4) as u32, t.accent_wash);
    cover(fb, a, id, rad);
    draw::outline(fb, a.tk(), rad, hair() as u32, t.border);

    let tx = a.right() + m.s6;
    let tw = r.right() - tx;
    let (x, room) = (tx.max(0) as u32, tw.max(0) as u32);
    text::left(fb, x, a.y + m.s3, "NOW PLAYING", t.accent, label);
    let title = text::truncate(&v.title, room, display);
    text::left(fb, x, a.y + m.s3 + line_h(label), &title, t.text, display);
    let artist = text::truncate(&v.artist, room, m.item);
    text::left(fb, x, a.y + m.s3 + line_h(label) + line_h(display), &artist, t.text_2, m.item);

    let cy = a.y + m.s3 + line_h(label) + line_h(display) + line_h(m.item) + m.s4;
    Pill::new(&v.format).draw(fb, Rect::new(tx, cy, tw, px(30)).tk());

    let (sy, sh) = (cy + px(30) + m.s6, px(60));
    stat(fb, Rect::new(tx, sy, tw / 2, sh), "Elapsed", &mmss(v.pos_ms));
    stat(fb, Rect::new(tx + tw / 2, sy, tw / 2, sh), "Duration", &mmss(v.dur_ms));

    let wr = wave_rect(r);
    if wr.h > px(20) {
        text::left(fb, wr.x.max(0) as u32, wr.y, "WAVEFORM", t.muted, label);
        let bars = Rect::new(wr.x, wr.y + line_h(label), wr.w, wr.h - line_h(label));
        waveform(fb, bars, &w.buckets, v.pos_ms as u64, v.dur_ms.max(1) as u64);
    }
}
