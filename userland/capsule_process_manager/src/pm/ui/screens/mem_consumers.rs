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
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::{clip, mono_right};
use nonos_toolkit::widgets::{Card, Rect};

use crate::pm::format::mem_human;
use crate::pm::format_mem::{mapped, share_1dp};
use crate::pm::state::{Row, State};

use super::super::metrics::{card_metrics, metrics};
use super::super::tint::state_tint;
use super::super::{bars, text};

const NAME_W: u32 = 214;
const VALUE_W: u32 = 96;
const BAR_GAP: u32 = 14;

// Bars scale to the largest process; the share of installed RAM is printed.
pub fn paint(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32) {
    let (r, c, px) = (roles(), card_metrics(), metrics().body_px);
    let (pad, head_h, row_h) = (c.panel_pad, c.panel_head_h, c.bar_row_h);
    Card::new().draw(fb, Rect { x, y, w, h });
    let top = text::centred_top(y, head_h, px);
    text::left(fb, x + pad, top, b"TOP CONSUMERS", r.text, px);
    let right = x + w.saturating_sub(pad);
    text::right(fb, right, top, b"share of RAM, mmapped, resident", r.muted, px);
    let mut rows = state.filtered();
    rows.sort_unstable_by(|a, b| b.mem_kb.cmp(&a.mem_kb));
    let den = rows.first().map(|row| row.mem_kb).unwrap_or(0).max(1);
    let slots = (h.saturating_sub(head_h + pad) / row_h) as usize;
    for (slot, row) in rows.iter().take(slots).enumerate() {
        entry(fb, x, y + head_h + slot as u32 * row_h, w, (row, den), state.sys.mem_total_kb);
    }
}

fn entry(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, row: (&Row, u64), ram_kb: u64) {
    let ((row, den), r) = (row, roles());
    let (m, c, s) = (metrics(), card_metrics(), scale());
    let (name_w, value_w, gap) = (s.px(NAME_W), s.px(VALUE_W), s.px(BAR_GAP));
    let top = text::centred_top(y, c.bar_row_h, m.body_px);
    let name = clip(text::valid(row.name()), name_w, m.body_px).as_bytes();
    text::left(fb, x + c.panel_pad, top, name, r.text_2, m.body_px);
    let mut buf = [0u8; 20];
    let n = mem_human(row.mem_kb, &mut buf);
    let right = x + w.saturating_sub(c.panel_pad);
    mono_right(fb, right, top as i32, text::valid(&buf[..n]), r.text, m.num_px);
    let shown = mapped(row, &mut buf);
    mono_right(
        fb,
        right.saturating_sub(value_w),
        top as i32,
        text::valid(shown),
        r.muted,
        m.num_px,
    );
    let n = share_1dp(row.mem_kb, ram_kb, &mut buf);
    mono_right(
        fb,
        right.saturating_sub(value_w * 2),
        top as i32,
        text::valid(&buf[..n]),
        r.muted,
        m.num_px,
    );
    let bx = x + c.panel_pad + name_w + gap;
    let bw = right.saturating_sub(value_w * 3 + gap).saturating_sub(bx);
    let by = y + (c.bar_row_h - c.bar_h) / 2;
    bars::hbar(fb, bx, by, bw, c.bar_h, row.mem_kb, den, state_tint(row.state));
}
