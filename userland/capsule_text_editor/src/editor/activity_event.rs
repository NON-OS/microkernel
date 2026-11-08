// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 NONOS
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! The activity bar on every screen: its frame, and its pointer fed to its
//! controls. Its rows act on press, as they always have, and an open menu or
//! panel takes the pointer from them.

use nonos_app_skeleton::{feed, EventOutcome, InputEvent, InputKind, PaintBuffer};
use nonos_toolkit::widgets::{Rect, Step};

use super::activity_bar::{paint_rail, ROWS};
use super::app::Editor;
use super::layout::{ACTIVITY_W, TITLEBAR_H};
use super::screen::Screen;

const ROW_TOP: u32 = TITLEBAR_H + 12;
const ROW_PITCH: u32 = 44;

impl Editor {
    /// Paints the bar down the window's left edge.
    pub(super) fn paint_activity(&mut self, fb: &mut PaintBuffer) {
        let column = Rect { x: 0, y: 0, w: ACTIVITY_W, h: fb.height };
        let band = Rect { y: ROW_TOP, h: ROW_PITCH * ROWS.len() as u32, ..column };
        let on_show = (self.screen, self.sidebar_open);
        paint_rail(fb, column, band, on_show, &mut self.act_ctl);
    }

    /// Whether a menu or panel lies over the window and takes the pointer.
    pub(super) fn covered(&self) -> bool {
        let menu = self.menu.is_some() || self.mb_open.is_some() || self.rb_open.is_some();
        menu || self.panel.is_some()
    }

    /// Feeds a pointer event to the bar. While covered the pointer has left
    /// it, so no row lights or acts beneath a menu.
    pub(super) fn activity_step(&mut self, event: &InputEvent) -> Step<Screen> {
        if self.covered() {
            return self.act_ctl.left();
        }
        feed(&mut self.act_ctl, event)
    }

    /// What the bar makes of `step`: a row's screen on a press over it, and a
    /// press anywhere down the bar's column taken, row or not.
    pub(super) fn activity_event(
        &mut self,
        event: &InputEvent,
        step: Step<Screen>,
    ) -> Option<EventOutcome> {
        let column = (0..ACTIVITY_W as i32).contains(&event.x);
        match step {
            Step::Act(target) => Some(self.activity_press(target)),
            _ if column && event.kind == InputKind::ButtonDown => Some(EventOutcome::Repaint),
            _ => None,
        }
    }
}
