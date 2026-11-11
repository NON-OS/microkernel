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

//! The decoded-peak waveform. Buckets left of the playhead take the cyan
//! gradient, buckets right of it the muted ground, so the bar doubles as a
//! progress read-out without a second control.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::derive::mix;
use nonos_toolkit::theme::{roles, Opaque};
use nonos_toolkit::widgets::draw;

use crate::ui::geometry::Rect;
use crate::ui::metrics::{metrics, px};

const BAR: i32 = 5;

/// The low end of the accent ramp: the accent a step towards the ground.
fn accent_dim() -> u32 {
    let r = roles();
    mix(r.accent.argb(), r.bg.argb(), 80)
}

pub fn waveform(fb: &mut PaintBuffer, r: Rect, buckets: &[u8], num: u64, den: u64) {
    if buckets.is_empty() || r.w <= 0 {
        return;
    }
    let (bar_w, rad) = (px(BAR), px(2) as u32);
    let step = bar_w + metrics().s1 / 2;
    let n = (r.w / step).max(1);
    let head = if den == 0 { 0 } else { (num * n as u64 / den) as i32 };
    let (t, dim) = (roles(), accent_dim());
    for i in 0..n {
        let b = buckets[(i as usize * buckets.len() / n as usize).min(buckets.len() - 1)];
        let h = ((b as i32 * r.h / 255).max(px(2)) / 2) * 2;
        let bar = Rect::new(r.x + i * step, r.cy() - h / 2, bar_w, h);
        let c = if i <= head { mix(dim, t.accent.argb(), (i * 255 / n) as u8) } else { t.muted.argb() };
        draw::round(fb, bar.tk(), rad, Opaque::new(c));
    }
}
