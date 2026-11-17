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

//! The kernel's own pace: load averages, then each per-second counter as
//! its last minute of shape with the current figure beside it. Every
//! trace is scaled to its own peak, so a quiet machine still shows one.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::mono_right;

use crate::pm::format_sys::rate_human;
use crate::pm::state::{RateRing, State};

use super::super::chrome::Rect;
use super::super::metrics::{card_metrics, metrics};
use super::super::{spark, text};
use super::cpu::panel;

const TRACE_PAD: u32 = 10;
const GAP: u32 = 14;
const LABEL_W: u32 = 118;
const VALUE_W: u32 = 60;

pub(super) fn kernel(state: &State, fb: &mut PaintBuffer, r: &Rect) {
    let (c, mut y) = (roles(), panel(fb, r, b"KERNEL"));
    let (m, cm, s) = (metrics(), card_metrics(), scale());
    let (spark_h, trace_pad) = (cm.spark_h, s.px(TRACE_PAD));
    let row_h = spark_h + trace_pad;
    let w = r.w.saturating_sub(cm.panel_pad * 2);
    let x = r.x + cm.panel_pad;
    let right = x + w;
    let mut buf = [0u8; 40];
    y = super::cpu_kernel_head::head(state, fb, x, y, right);
    let h = &state.history;
    let rows: [(&[u8], &RateRing); 4] = [
        (b"syscalls / s", &h.syscalls),
        (b"messages / s", &h.messages),
        (b"switches / s", &h.switches),
        (b"interrupts / s", &h.interrupts),
    ];
    for (label, ring) in rows {
        let top = text::centred_top(y, row_h, m.body_px);
        text::left(fb, x, top, label, c.text_2, m.body_px);
        let n = rate_human(ring.latest(), &mut buf);
        mono_right(fb, right, top as i32, text::valid(&buf[..n]), c.accent, m.num_px);
        let sx = x + s.px(LABEL_W) + s.px(GAP);
        let sw = right.saturating_sub(s.px(VALUE_W) + s.px(GAP)).saturating_sub(sx);
        spark::rate(
            fb,
            sx,
            y + trace_pad / 2,
            sw,
            spark_h,
            ring,
            ring.peak() + ring.peak() / 4 + 1,
            c.accent,
        );
        y += row_h;
    }
}
