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

//! Painting a split window: each pane is its tab over its own half, sized to
//! that half so a program in it sees its real grid, with the divider between
//! them and a line of the accent over the pane that has the keys.

use nonos_app_skeleton::PaintBuffer;

use super::super::types::Terminal;
use crate::layout::Layout;
use crate::paint::metrics::Metrics;
use crate::term::split::{halves, sides, DIVIDER};

/// `l` as seen from the window, its pane `dx` from the left.
pub fn shifted(mut l: Layout, dx: u32) -> Layout {
    for r in [&mut l.left_rail, &mut l.body, &mut l.input, &mut l.footer] {
        r.x += dx;
    }
    l
}

impl Terminal {
    pub(super) fn paint_split(&mut self, fb: &mut PaintBuffer, other: usize) {
        let (left, right) = sides(self.active, other);
        let ((lx, lw), (rx, rw)) = halves(fb.width);
        let h = fb.height;
        for (ix, x, w) in [(left, lx, lw), (right, rx, rw)] {
            self.paint_pane(&mut fb.sub(x, 0, w, h), ix, x);
        }
        let t = crate::term::theme::profiles::by_index(self.theme);
        fb.fill_rect(lw, 0, DIVIDER, h, t.chrome_edge);
        let (fx, fw) = if self.active == left { (lx, lw) } else { (rx, rw) };
        fb.fill_rect(fx, 0, fw, 2, t.accent);
    }

    /// Where the tab in front drew its cells and bands, for the pointer.
    pub(super) fn record_front(&mut self, l: Layout, m: Metrics, dx: u32) {
        let s = self.cur_ref();
        let owned = s.scrollback.vt.alt_active() || s.fg_running;
        self.cells = Some(super::super::pointer::CellGeom {
            x: dx + l.body.x + crate::paint::TEXT_LEFT,
            y: l.body.y,
            adv: m.adv,
            lh: m.lh,
            pad: crate::paint::TEXT_LEFT,
            shell_rows: (l.body.h / m.lh.max(1)) as usize,
            owned,
        });
        self.layout = Some(shifted(l, dx));
    }
}
