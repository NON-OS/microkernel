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
use crate::ui::metrics::{line_h, DISPLAY, HERO, ITEM, LABEL, R_CARD, S3, S4, S6};
use crate::ui::widget::waveform;
use crate::waveform::Waveform;

const ART: i32 = 300;

fn art_side(r: Rect) -> i32 {
    (r.w * 42 / 100).clamp(180, ART)
}

pub fn art_rect(r: Rect) -> Rect {
    let s = art_side(r);
    Rect::new(r.x, r.y + S4, s, s)
}

pub fn wave_rect(r: Rect) -> Rect {
    let a = art_rect(r);
    Rect::new(r.x, a.bottom() + S6, r.w, (r.bottom() - a.bottom() - S6 - S4).clamp(0, 120))
}

fn stat(fb: &mut PaintBuffer, r: Rect, label: &str, value: &str) {
    let (t, x) = (roles(), r.x.max(0) as u32);
    text::left(fb, x, r.y, &label.to_ascii_uppercase(), t.muted, LABEL);
    text::left(fb, x, r.y + line_h(LABEL), value, t.text, HERO);
}

pub fn paint(fb: &mut PaintBuffer, r: Rect, v: &PlayerView, w: &Waveform, id: &str) {
    let (a, t) = (art_rect(r), roles());
    draw::round(fb, a.inset(-S3).tk(), R_CARD + 4, t.accent_wash);
    cover(fb, a, id, R_CARD);
    draw::outline(fb, a.tk(), R_CARD, 1, t.border);

    let tx = a.right() + S6;
    let tw = r.right() - tx;
    let (x, room) = (tx.max(0) as u32, tw.max(0) as u32);
    text::left(fb, x, a.y + S3, "NOW PLAYING", t.accent, LABEL);
    let title = text::truncate(&v.title, room, DISPLAY);
    text::left(fb, x, a.y + S3 + line_h(LABEL), &title, t.text, DISPLAY);
    let artist = text::truncate(&v.artist, room, ITEM);
    text::left(fb, x, a.y + S3 + line_h(LABEL) + line_h(DISPLAY), &artist, t.text_2, ITEM);

    let cy = a.y + S3 + line_h(LABEL) + line_h(DISPLAY) + line_h(ITEM) + S4;
    Pill::new(&v.format).draw(fb, Rect::new(tx, cy, tw, 30).tk());

    let sy = cy + 30 + S6;
    stat(fb, Rect::new(tx, sy, tw / 2, 60), "Elapsed", &mmss(v.pos_ms));
    stat(fb, Rect::new(tx + tw / 2, sy, tw / 2, 60), "Duration", &mmss(v.dur_ms));

    let wr = wave_rect(r);
    if wr.h > 20 {
        text::left(fb, wr.x.max(0) as u32, wr.y, "WAVEFORM", t.muted, LABEL);
        let bars = Rect::new(wr.x, wr.y + line_h(LABEL), wr.w, wr.h - line_h(LABEL));
        waveform(fb, bars, &w.buckets, v.pos_ms as u64, v.dur_ms.max(1) as u64);
    }
}
