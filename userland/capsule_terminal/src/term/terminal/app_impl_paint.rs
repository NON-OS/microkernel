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

use crate::term::prefs::RAIL_VISIBLE;
use nonos_app_skeleton::PaintBuffer;

use super::types::Terminal;

#[path = "app_impl_paint_split.rs"]
mod app_impl_paint_split;

impl Terminal {
    pub(super) fn paint_inner(&mut self, fb: &mut PaintBuffer) {
        self.width = fb.width;
        if !self.paint_line_only(fb) {
            match self.split.filter(|&s| s < self.tabs.len() && s != self.active) {
                Some(other) => self.paint_split(fb, other),
                None => self.paint_pane(fb, self.active, 0),
            }
        }
        // The key that asked for this frame has reached the screen.
        if self.key_at > 0 {
            let ms = (nonos_libc::mk_uptime_ms() - self.key_at).clamp(0, u32::MAX as i64);
            self.cur().key_lat.push(ms as u32);
            self.key_at = 0;
        }
    }

    /// Tab `ix` over the whole of `fb`, which sits `dx` from the window's
    /// left; the tab in front also leaves where its cells fell, for the pointer.
    pub(super) fn paint_pane(&mut self, fb: &mut PaintBuffer, ix: usize, dx: u32) {
        let theme = crate::term::theme::profiles::by_index(self.theme);
        let rail_open = self.prefs.rails & RAIL_VISIBLE != 0;
        let (cols, rows, m) = crate::paint::grid_size(fb, self.font_scale, rail_open);
        let theme_ix = self.theme;
        let sb = &mut self.tabs[ix].scrollback;
        let resized = sb.fit(cols, rows, m.adv, m.lh);
        sb.follow_theme(theme_ix, theme);
        if resized {
            crate::jobs::tty::resized(&self.tabs[ix]);
        }
        let scene = crate::paint::Scene {
            tabs: &self.tabs,
            active: ix,
            rail: &self.rail,
            projects: self.prefs.project_slice(),
            monitor: self.prefs.rails & 1 == 0,
            scroll: self.rail_scroll,
            pal: &self.palette,
            rail_open,
        };
        let l = crate::paint::paint_tabs(&scene, fb, theme, self.font_scale);
        if ix == self.active {
            self.record_front(l, m, dx);
        }
    }
}
