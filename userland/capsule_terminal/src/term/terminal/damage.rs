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

//! A key that changed the input line and nothing else repaints the input row
//! alone: the frame reports that row as its damage and paints only it, where
//! every key used to redraw the whole window. Anything else on screen that
//! moved (a block, the scrollback, a panel, a mode, the tab, the size) makes
//! the fingerprint differ, and the frame is drawn whole.

use nonos_app_skeleton::PaintBuffer;

use super::types::Terminal;
use crate::paint::draw_input_line::draw_input_line;
use crate::layout::Rect;
use crate::paint::metrics::Metrics;

impl Terminal {
    /// Everything the frame draws besides the line itself.
    pub(super) fn frame_key(&self) -> [usize; 12] {
        let s = self.cur_ref();
        let vt = &s.scrollback.vt;
        let open = (s.search.is_some() as usize) | (s.find.is_some() as usize) << 1;
        let more = (s.theme_pick.is_some() as usize) | (self.palette.open as usize) << 1;
        [
            self.active,
            self.split.map_or(usize::MAX, |i| i),
            s.blocks.len(),
            vt.history_len(),
            vt.view_offset(),
            open | more << 2 | (s.fg_running as usize) << 4 | (s.fresh as usize) << 5,
            s.completions.len() | (s.sel.is_some() as usize) << 16,
            self.theme as usize,
            self.font_scale as usize,
            self.dropped as usize,
            s.last_status as usize,
            self.prefs.rails as usize,
        ]
    }

    /// The input row while the last key changed only the line; it is the
    /// next frame's damage, and that frame paints the row alone.
    pub(super) fn take_damage_inner(&mut self) -> Option<Rect> {
        let line = core::mem::take(&mut self.line_only) && self.split.is_none();
        let row = self.layout.map(|l| l.input).filter(|_| line)?;
        self.partial = true;
        Some(row)
    }

    /// Paint the input row alone, when the frame was asked for that; false
    /// when the frame is to be drawn whole.
    pub(super) fn paint_line_only(&mut self, fb: &mut PaintBuffer) -> bool {
        let Some(l) = self.layout.filter(|_| core::mem::take(&mut self.partial)) else {
            return false;
        };
        let theme = crate::term::theme::profiles::by_index(self.theme);
        let m = Metrics::new(fb, self.font_scale);
        draw_input_line(self.cur_ref(), fb, l.input, m, theme);
        true
    }
}
