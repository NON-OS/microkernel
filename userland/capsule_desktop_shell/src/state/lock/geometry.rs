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

//! Where the lock screen puts its clock, date, field and note on one
//! output, in drawing pixels. `u` is drawing pixels per logical pixel.

pub type Rect4 = (u32, u32, u32, u32);

const FIELD_W: u32 = 420;
const FIELD_H: u32 = 44;
const SIDE: u32 = 24;
const TIME_ABOVE: u32 = 190;
const DATE_ABOVE: u32 = 72;
const NOTE_BELOW: u32 = 18;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Places {
    pub time_y: u32,
    pub date_y: u32,
    pub field: Rect4,
    pub note_y: u32,
}

pub fn places(out: Rect4, u: u32) -> Places {
    let (ox, oy, ow, oh) = out;
    let u = u.max(1);
    let w = (FIELD_W * u).min(ow.saturating_sub(2 * SIDE * u)).max(u.min(ow));
    let h = (FIELD_H * u).min(oh);
    let x = ox + (ow - w) / 2;
    let y = oy + oh.saturating_sub(h) / 2;
    Places {
        time_y: y.saturating_sub(TIME_ABOVE * u).max(oy),
        date_y: y.saturating_sub(DATE_ABOVE * u).max(oy),
        field: (x, y, w, h),
        note_y: (y + h + NOTE_BELOW * u).min(oy + oh),
    }
}

/// The rows a tick of the locked clock changes on output `out`: from the
/// time's top through the date and the field to `note_h` pixels below the
/// note's top (its count down), clipped to the output.
pub fn tick_band(out: Rect4, u: u32, note_h: u32) -> Rect4 {
    let p = places(out, u);
    let end = out.1.saturating_add(out.3);
    let bottom = p.note_y.saturating_add(note_h).min(end);
    (out.0, p.time_y, out.2, bottom.saturating_sub(p.time_y))
}

/// One box over every output's tick band: what a clock tick shows while
/// locked. With no output known the curtain is drawn over `whole`, and so is
/// the band.
pub fn tick_bounds(outputs: &[Rect4], whole: Rect4, u: u32, note_h: u32) -> Rect4 {
    let outs = if outputs.is_empty() { core::slice::from_ref(&whole) } else { outputs };
    let mut b: Option<(u32, u32, u32, u32)> = None;
    for &o in outs {
        let (x, y, w, h) = tick_band(o, u, note_h);
        let (r, s) = (x.saturating_add(w), y.saturating_add(h));
        b = Some(match b {
            None => (x, y, r, s),
            Some((x0, y0, r0, s0)) => (x0.min(x), y0.min(y), r0.max(r), s0.max(s)),
        });
    }
    let (x0, y0, r, s) = b.unwrap_or((whole.0, whole.1, whole.0, whole.1));
    (x0, y0, r - x0, s - y0)
}

/// The field on every output: where the pointer is drawn while locked.
pub fn fields(outputs: &[Rect4], u: u32) -> alloc::vec::Vec<Rect4> {
    outputs.iter().map(|&o| places(o, u).field).collect()
}

/// Whether (x, y) is inside `r`.
pub fn inside(r: Rect4, x: u32, y: u32) -> bool {
    x >= r.0 && y >= r.1 && x - r.0 < r.2 && y - r.1 < r.3
}

const PANEL_W: u32 = 560;
const PANEL_H: u32 = 340;
const PAD: u32 = 22;
const BTN_W: u32 = 130;
const BTN_H: u32 = 36;

/// The passphrase prompt on the first output: its panel, field and two
/// buttons (the step's own, then Not now or Cancel).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PromptPlaces {
    pub panel: Rect4,
    pub field: Rect4,
    pub primary: Rect4,
    pub secondary: Rect4,
}

pub fn prompt(out: Rect4, u: u32) -> PromptPlaces {
    let (ox, oy, ow, oh) = out;
    let u = u.max(1);
    let (pw, ph) = ((PANEL_W * u).min(ow), (PANEL_H * u).min(oh));
    let (px, py) = (ox + (ow - pw) / 2, oy + (oh - ph) / 2);
    let pad = (PAD * u).min(pw / 4);
    let (bw, bh) = ((BTN_W * u).min(pw / 2), (BTN_H * u).min(ph / 4));
    let by = py + ph.saturating_sub(bh + pad);
    let mid = px + pw / 2;
    let fh = (FIELD_H * u).min(ph / 4);
    let fy = by.saturating_sub(fh + pad * 2);
    PromptPlaces {
        panel: (px, py, pw, ph),
        field: (px + pad, fy, pw.saturating_sub(2 * pad), fh),
        primary: (mid.saturating_sub(bw + pad / 2), by, bw, bh),
        secondary: (mid + pad / 2, by, bw, bh),
    }
}
