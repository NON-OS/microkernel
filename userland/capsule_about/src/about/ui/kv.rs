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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::{centre_top, left, mono_left, truncate, truncate_mono};

use super::metrics::metrics;
use super::mono::valid;

pub fn row_h() -> u32 {
    metrics().kv_row_h
}

// A label/value pair on one baseline. The label column is fixed so a stack of
// rows reads as a table without drawing one; the value is measured and cut to
// what is left, never by glyph count, because the body face is proportional.
// Numeric and hash-like values go through the mono face so digits align down
// the column. The row is placed in i32 so one that has left the pane simply does
// not land rather than being clamped to its top edge.
pub fn kv(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, label: &[u8], value: &[u8], num: bool) {
    let (r, m) = (roles(), metrics());
    let (body, mono) = (m.body_px, m.num_px);
    let top = y + centre_top(0, m.kv_row_h, body);
    let label_w = m.kv_label_w.min(w);
    let cut = truncate(valid(label), label_w.saturating_sub(m.kv_gap), body);
    left(fb, x, top, &cut, r.muted, body);
    let value_x = x + label_w;
    let value_w = w.saturating_sub(label_w);
    if num {
        let cut = truncate_mono(valid(value), value_w, mono);
        mono_left(fb, value_x, top, &cut, r.text_2, mono);
        return;
    }
    let cut = truncate(valid(value), value_w, body);
    left(fb, value_x, top, &cut, r.text_2, body);
}

// A label over its value. Long values that would be cut to nothing in a kv row's
// share of a narrow card get the card's whole width here instead.
pub fn pair(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, label: &[u8], value: &[u8], num: bool) {
    let (r, m) = (roles(), metrics());
    let cut = truncate(valid(label), w, m.body_px);
    left(fb, x, y, &cut, r.muted, m.body_px);
    let vy = y + m.pair_line as i32;
    if num {
        let cut = truncate_mono(valid(value), w, m.num_px);
        mono_left(fb, x, vy, &cut, r.text_2, m.num_px);
        return;
    }
    let cut = truncate(valid(value), w, m.body_px);
    left(fb, x, vy, &cut, r.text_2, m.body_px);
}
