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

//! The Settings painter. Every fact on this screen is read back from the
//! capsule itself: the directory the library scanned, the count it found, the
//! resampler output rate and the decoders that were compiled in.

extern crate alloc;

use alloc::format;
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::derive::with_alpha;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Card, Progress, Row, SectionHeader, Separator, Switch, Tone};

use crate::decode::DECODERS;
use crate::library::{Library, Queue};
use crate::model::PlayerView;
use crate::resample::OUT_RATE;
use crate::ui::geometry::{page_head, Rect};
use crate::ui::hover::Lit;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::marks::slider;
use crate::ui::metrics::{hair, line_h, metrics, px};
use crate::ui::target::Target;

use super::settings::{
    fact_h, facts_body, facts_rect, meter_rect, panel_rect, tiles_rect, toggle_row, volume_rect,
    TOGGLES,
};

/// The line top that centres one line at `px` down `r`.
fn mid(r: Rect, px: f32) -> i32 {
    text::centre_top(r.tk().y, r.tk().h, px)
}

fn rule(fb: &mut PaintBuffer, row: Rect) {
    let hr = hair();
    Separator::new().draw(fb, Rect::new(row.x, row.bottom() - hr, row.w, hr).tk());
}

fn fact(fb: &mut PaintBuffer, r: Rect, i: i32, key: &str, value: &str) {
    let h = (r.h / 4).min(fact_h()).max(1);
    let row = r.row(i * h, h);
    let (t, m) = (roles(), metrics());
    text::left(fb, row.tk().x, mid(row, m.label), key, t.muted, m.label);
    text::right(fb, row.right().max(0) as u32, mid(row, m.item), value, t.text, m.item);
    rule(fb, row);
}

fn playback(fb: &mut PaintBuffer, r: Rect, v: &PlayerView, lit: &Lit) {
    Card::new().title("Playback").draw(fb, panel_rect(r).tk());
    let states = [v.shuffle, v.repeat, v.muted];
    for (i, label) in TOGGLES.iter().enumerate() {
        let (row, switch) = (toggle_row(r, i).tk(), Switch::new(states[i]));
        let line = Row::new(label).trailing(switch.size().0).state(lit.of(Target::Toggle(i)));
        line.draw(fb, row);
        switch.draw(fb, line.trailing_rect(row));
        rule(fb, toggle_row(r, i));
    }
    Row::new("Volume").draw(fb, toggle_row(r, 3).tk());
    slider(fb, volume_rect(r), v.volume_q15.max(0) as u32, 32768, lit.of(Target::Volume));
}

/// A counter's tile: its glyph, its name and its value, on a card.
fn stat_tile(fb: &mut PaintBuffer, icons: &Icons, r: Rect, value: &str, label: &str, g: Glyph) {
    let (t, m, side) = (roles(), metrics(), px(20));
    let (lp, dp) = (m.label, m.data);
    Card::new().draw(fb, r.tk());
    let inner = r.pad(m.s4, (r.h - side).max(0) / 2);
    let mark = Rect::new(inner.x, inner.y, side, side);
    icons.draw(fb, mark, g, with_alpha(t.accent.argb(), 0xCC));
    let lx = inner.x + side + m.s3;
    let name = label.to_ascii_uppercase();
    text::left(fb, lx.max(0) as u32, inner.y + (side - line_h(lp)) / 2, &name, t.muted, lp);
    let vr = Rect::new(lx, inner.y + (side - line_h(dp)) / 2, inner.right() - lx, line_h(dp));
    text::right(fb, vr.right().max(0) as u32, mid(vr, dp), value, t.text, dp);
}

/// How much of the library is queued, as a bar on a card.
fn storage_meter(fb: &mut PaintBuffer, r: Rect, used: u32, total: u32, used_label: &str) {
    let (t, m) = (roles(), metrics());
    let label = m.label;
    Card::new().draw(fb, r.tk());
    let inner = r.pad(m.s4, m.s4);
    text::left(fb, inner.tk().x, inner.y, "STORAGE", t.muted, label);
    let top = text::centre_top(inner.tk().y, line_h(label) as u32, label);
    text::right(fb, inner.right().max(0) as u32, top, used_label, t.text, label);
    let bar = Rect::new(inner.x, inner.bottom() - px(10), inner.w, px(8));
    let pct = if total == 0 { 0 } else { (used * 100 / total).min(100) };
    let tone = [Tone::Ok, Tone::Accent, Tone::Warn][usize::from(pct > 40) + usize::from(pct > 80)];
    Progress::new(pct, 100).tone(tone).draw(fb, bar.tk());
    let rule = Rect::new(bar.x, bar.y - m.s3 - px(2), inner.w, hair());
    Separator::new().soft().draw(fb, rule.tk());
}

fn counters(fb: &mut PaintBuffer, icons: &Icons, r: Rect, lib: &Library, q: &Queue) {
    let t = tiles_rect(r);
    let queued = q.items().len() as u32;
    let total = lib.tracks.len() as u32;
    let s2 = metrics().s2;
    let h = (t.h - s2 * 2) / 3;
    let cell = |i: i32| Rect::new(t.x, t.y + i * (h + s2), t.w, h);
    stat_tile(fb, icons, cell(0), &format!("{total}"), "Indexed", Glyph::Note);
    stat_tile(fb, icons, cell(1), &format!("{queued}"), "Queued", Glyph::Plus);
    stat_tile(fb, icons, cell(2), &format!("{}", DECODERS.len()), "Decoders", Glyph::Check);
    storage_meter(fb, meter_rect(r), queued, total.max(1), &format!("{queued} queued"));
}

pub fn paint(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    v: &PlayerView,
    lib: &Library,
    q: &Queue,
    lit: &Lit,
) {
    let note = "Playback, and what this player can read.";
    SectionHeader::new("Settings").note(note).draw(fb, page_head(r).tk());
    playback(fb, r, v, lit);
    Card::new().title("This capsule").draw(fb, facts_rect(r).tk());
    let f = facts_body(r);
    fact(fb, f, 0, "Library folder", crate::fetch::name::MUSIC_DIR);
    fact(fb, f, 1, "Tracks found", &format!("{}", lib.tracks.len()));
    fact(fb, f, 2, "Output rate", &format!("{OUT_RATE}"));
    fact(fb, f, 3, "Decoders", &DECODERS.join(", "));
    counters(fb, icons, r, lib, q);
}
