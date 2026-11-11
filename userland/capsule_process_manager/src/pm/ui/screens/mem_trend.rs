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
use nonos_toolkit::widgets::text::mono_right;
use nonos_toolkit::widgets::{Card, Rect, Tone};

use crate::pm::format::mem_human;
use crate::pm::state::{Ring, State};

use super::super::metrics::{card_metrics, metrics};
use super::super::{spark, text};

// Falling resident memory is the healthy direction, so the sign picks the tint
// rather than the size: ok when the newest sample sits below the oldest one the
// ring still holds, warn when it has grown, plain (muted) before there is a
// history.
pub fn delta(ring: &Ring) -> (i64, Tone) {
    if ring.is_empty() {
        return (0, Tone::Plain);
    }
    let newest = ring.mem_at(ring.len() - 1) as i64;
    let moved = newest - ring.oldest_mem() as i64;
    (moved, if moved > 0 { Tone::Warn } else { Tone::Ok })
}

pub fn paint(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32) {
    let (r, m, c) = (roles(), metrics(), card_metrics());
    let (pad, head_h) = (c.panel_pad, c.panel_head_h);
    Card::new().draw(fb, Rect { x, y, w, h });
    let top = text::centred_top(y, head_h, m.body_px);
    text::left(fb, x + pad, top, b"RESIDENT TREND", r.text, m.body_px);
    let ring = &state.history.total;
    let (moved, tone) = delta(ring);
    let tint = tone.colour(&r);
    let mut buf = [0u8; 20];
    buf[0] = if moved < 0 { b'-' } else { b'+' };
    let n = 1 + mem_human(moved.unsigned_abs(), &mut buf[1..]);
    mono_right(fb, x + w.saturating_sub(pad), top as i32, text::valid(&buf[..n]), tint, m.num_px);
    let plot_h = h.saturating_sub(head_h + pad);
    spark::mem(fb, x + pad, y + head_h, w.saturating_sub(pad * 2), plot_h, ring, tint);
}
