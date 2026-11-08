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
use nonos_toolkit::widgets::Scrolled;

use super::rail_fmt::{load3_into, mib_into, pct, uptime};
use super::rail_geom::SPARK_H;
use super::rail_metric::{one, pair};
use super::rail_text::{bar, gap, head, stat, BAR_H};
use super::spark::draw_spark;
use crate::layout::Rect;
use crate::rail::Rail;

pub use super::rail_geom::sys_h as height;

/// What the machine is doing now and what it has been doing, from content
/// row `y` of a column scrolled by `off`. Swap has no source on NONOS and says
/// so; the CPU figure is real and carries both the bar and the trend beneath it.
pub fn draw(fb: &mut PaintBuffer, off: u32, x: u32, y: u32, w: u32, rail: &Rail) {
    let s = &rail.sample;
    let mut b = [0u8; 48];
    let sc = &mut Scrolled::new(fb, off);
    let mut y = head(sc, x, y, w, "System");
    y = stat(sc, x, y, w, "Uptime", uptime(&mut b, s.uptime_ms));
    y = stat(sc, x, y, w, "Load", one(&mut b, s.load_avg, load3_into));
    y = stat(sc, x, y, w, "CPU", pct(&mut b, s.cpu_pct));
    bar(sc, x, y, w, s.cpu_pct);
    y += scale().px(BAR_H) + gap();
    let spark_h = scale().px(SPARK_H);
    if let Some(top) = y.checked_sub(off) {
        let r = Rect { x, y: top, w, h: spark_h };
        draw_spark(fb, r, rail.spark.slice(), rail.spark.start(), roles().accent.argb());
    }
    y += spark_h + gap();
    let sc = &mut Scrolled::new(fb, off);
    y = stat(sc, x, y, w, "Memory", pair(&mut b, s.mem.used_kb, s.mem.total_kb, mib_into));
    stat(sc, x, y, w, "Swap", one(&mut b, s.mem.swap_used_kb, mib_into));
}
