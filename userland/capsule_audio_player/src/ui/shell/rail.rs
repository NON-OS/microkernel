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

//! The right rail: the cover of what is playing, its scrubber and transport,
//! the volume trim and the up-next queue. `queue_row` is the one geometry
//! source (`rail_geom.rs`), so the hit-test lands on exactly the row the
//! painter drew.

extern crate alloc;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, text, Row, Separator, State as Look};

use crate::library::{Library, Queue};
use crate::model::PlayerView;
use crate::transport::State;
use crate::ui::art::cover;
use crate::ui::fmt::mmss;
use crate::ui::geometry::Rect;
use crate::ui::hover::Lit;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::marks::{glyph_on, play_disc, slider};
use crate::ui::metrics::{cap_h, hair, metrics, px};
use crate::ui::target::{Key, Target, Target::Rail};

use super::rail_geom::{
    art_rect, body, head_rect, play_rect, qrow_h, queue_row, queue_visible, seek_rect, small_rects,
    speaker_rect, vol_rect,
};

/// The line top that centres one line at `px` down `r`.
fn mid(r: Rect, px: f32) -> i32 {
    text::centre_top(r.tk().y, r.tk().h, px)
}

pub fn rail(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    lib: &Library,
    queue: &Queue,
    view: &PlayerView,
    lit: &Lit,
) {
    if r.w <= 0 {
        return;
    }
    let (th, m, hr) = (roles(), metrics(), hair());
    let (ink, muted) = (th.text, th.muted);
    draw::fill(fb, r.tk(), th.surface);
    draw::fill(fb, Rect::new(r.x, r.y, hr, r.h).tk(), th.border);

    let b = body(r);
    let (bx, bw) = (b.x.max(0) as u32, b.w.max(0) as u32);
    let art = art_rect(r);
    let title = view.title.as_str();
    let artist = view.artist.as_str();
    cover(fb, art, title, px(12) as u32);

    let ty = art.bottom() + m.s5;
    let head = text::truncate(title, bw, m.section);
    text::left(fb, bx, ty, &head, ink, m.section);
    let sub = text::truncate(artist, bw, m.secondary);
    text::left(fb, bx, ty + cap_h(m.section) * 2, &sub, th.text_2, m.secondary);

    let seek = seek_rect(r);
    slider(fb, seek, view.pos_ms, view.dur_ms.max(1), lit.of(Rail(Key::Seek)));
    let (label, times) = (m.label, Rect::new(b.x, seek.bottom() + m.s2, b.w, cap_h(m.label) * 2));
    text::left(fb, bx, mid(times, label), &mmss(view.pos_ms), muted, label);
    text::right(fb, bx + bw, mid(times, label), &mmss(view.dur_ms), muted, label);

    let g = if view.state == State::Playing { Glyph::Pause } else { Glyph::Play };
    play_disc(fb, icons, play_rect(r), (g, px(19), px(22)), lit.of(Rail(Key::Play)));
    let ([prev, next, shuffle, repeat], g18) = (small_rects(r), px(18));
    glyph_on(fb, icons, prev, (Glyph::Prev, g18, ink.argb()), lit.of(Rail(Key::Prev)));
    glyph_on(fb, icons, next, (Glyph::Next, g18, ink.argb()), lit.of(Rail(Key::Next)));
    let sh = if view.shuffle { th.accent } else { muted }.argb();
    let rp = if view.repeat { th.accent } else { muted }.argb();
    glyph_on(fb, icons, shuffle, (Glyph::Shuffle, g18, sh), lit.of(Rail(Key::Shuffle)));
    glyph_on(fb, icons, repeat, (Glyph::Repeat, g18, rp), lit.of(Rail(Key::Repeat)));

    let vol = vol_rect(r);
    let spk = if view.muted { th.danger } else { th.text_2 }.argb();
    glyph_on(fb, icons, speaker_rect(r), (Glyph::Speaker, px(16), spk), lit.of(Rail(Key::Mute)));
    let level = if view.muted { 0 } else { view.volume_q15.max(0) as u32 };
    slider(fb, vol, level, 32768, lit.of(Rail(Key::Volume)));

    let qh = head_rect(r);
    let secondary = m.secondary;
    text::left(fb, bx, mid(qh, secondary), "Up next", ink, secondary);
    Separator::new().draw(fb, Rect::new(b.x, qh.bottom() - hr, b.w, hr).tk());

    let items = queue.items();
    let shown = queue_visible(r).min(items.len());
    for i in 0..shown {
        let row = queue_row(r, i);
        let Some(t) = lib.get(items[i]) else { continue };
        let selected = t.title == view.title;
        Row::new("").state(Look { selected, ..lit.of(Target::Queue(i)) }).draw(fb, row.tk());
        let thumb = Rect::new(row.x + m.s2, row.cy() - px(16), px(32), px(32));
        cover(fb, thumb, &t.title, px(7) as u32);
        let tx = thumb.right() + m.s3;
        let tw = (row.right() - m.s3 - tx).max(0) as u32;
        let top = row.cy() - cap_h(secondary) - m.s1;
        text::left(fb, tx as u32, top, &text::truncate(&t.title, tw, secondary), ink, secondary);
        let artist = text::truncate(&t.artist, tw, m.label);
        text::left(fb, tx as u32, row.cy() + m.s1, &artist, muted, m.label);
    }
    if shown == 0 {
        let empty = Rect::new(b.x, queue_row(r, 0).y, b.w, qrow_h());
        text::centre(fb, empty.tk(), mid(empty, m.item), "Queue is empty", th.quiet, m.item);
    }
}
