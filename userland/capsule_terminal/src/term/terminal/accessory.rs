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

use nonos_app_skeleton::{EventOutcome, InputEvent, PaintBuffer};
use nonos_toolkit::design::scale::scale;

use super::types::Terminal;
use crate::paint::strip::nominal_w;
use crate::paint::strip_spot::Step;
use crate::paint::tab_label::with_labels;

const LIGHTS_RESERVE: u32 = 128;

impl Terminal {
    pub(super) fn accessory_w(&self) -> u32 {
        let want = nominal_w(self.tabs.len());
        match self.width.checked_sub(scale().px(LIGHTS_RESERVE)) {
            Some(room) => want.min(room),
            None => want,
        }
    }

    pub(super) fn paint_accessory_inner(&mut self, fb: &mut PaintBuffer) {
        self.strip.size = (fb.width, fb.height);
        let (active, sp) = (self.active, &self.strip);
        with_labels(&self.tabs, |n| crate::paint::draw_tab_bar(n, active, sp, fb));
    }

    pub(super) fn accessory_event(&mut self, event: InputEvent) -> EventOutcome {
        let let_go = self.palette_leave() | self.rail_leave(None);
        let (active, sp) = (self.active, &mut self.strip);
        match with_labels(&self.tabs, |n| sp.feed(n, active, &event)) {
            Step::Act(s) => self.strip_act(s),
            Step::Idle if let_go => EventOutcome::Repaint,
            Step::Repaint => EventOutcome::Repaint,
            Step::Idle => EventOutcome::Idle,
        }
    }

    /// The pointer reached the window off the strip. True when that unlit a
    /// control on it.
    pub(super) fn strip_leave(&mut self) -> bool {
        let (active, sp) = (self.active, &mut self.strip);
        with_labels(&self.tabs, |n| sp.leave(n, active))
    }
}
