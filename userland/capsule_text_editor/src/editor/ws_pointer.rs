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

//! Pointer routing for the shell chrome. Returns None when the event belongs
//! to the code pane, so the caller forwards it to the document engine.

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};
use nonos_toolkit::widgets::Step;

use super::app::Editor;
use super::layout::{activity_w, ribbon_h, tabbar_h, titlebar_h};
use super::menu_view::bounds;
use super::sb_menu::{menu_hit, open_menu};
use super::screen::Screen;
use super::shell::pane_x;
use super::strip::Spot;

// The input router posts button codes as bit numbers: 1 = left, 2 = right.
const BTN_RIGHT: u32 = 2;

impl Editor {
    pub(super) fn pointer_event(&mut self, event: &InputEvent) -> Option<EventOutcome> {
        let press = event.kind == InputKind::ButtonDown;
        let (x, y) = (event.x, event.y);
        let px = pane_x(self.sidebar_open) as i32;
        let tb_top = titlebar_h() as i32;
        let tb_bot = (titlebar_h() + tabbar_h()) as i32;
        let rb_bot = tb_bot + ribbon_h() as i32;
        let act = self.activity_step(event);
        let explorer = self.explorer_step(event);
        let panel = self.panel_step(event);
        let ribbon = self.ribbon_pointer(event);
        let strip = self.strip_pointer(event);
        if let Some(outcome) = self.menu_pointer(event) {
            return Some(outcome);
        }
        if ribbon.is_some() {
            return ribbon;
        }
        if strip.is_some() {
            return strip;
        }

        if let Some(outcome) = self.panel_event(event, panel) {
            return Some(outcome);
        }

        // An open context menu takes the whole next press: run the item under
        // the click, or just close on a miss.
        if press && self.menu.is_some() {
            let menu = self.menu.take();
            if let Some(m) = menu {
                if let Some(action) = menu_hit(&m, bounds(self.last_w, self.last_h), x, y) {
                    self.apply_menu_action(action, m.target);
                }
            }
            return Some(EventOutcome::Repaint);
        }

        // The menu bar's dropdown overlays the ribbon, so it is asked first and
        // takes the ribbon's dropdown down with it.
        if press && (self.mb_open.is_some() || y < tb_top) {
            self.rb_open = None;
            return Some(self.menubar_press(x, y));
        }

        let in_ribbon = x >= activity_w() as i32 && y >= tb_bot && y < rb_bot;
        if press && (self.rb_open.is_some() || in_ribbon) {
            return Some(self.ribbon_press(x, y));
        }

        if let Some(outcome) = self.activity_event(event, act) {
            return Some(outcome);
        }

        let in_sidebar = self.sidebar_open && x >= activity_w() as i32 && x < px && y >= tb_bot;
        if press && in_sidebar {
            let row = match explorer {
                Step::Act(vi) => Some(vi),
                _ => None,
            };
            if event.code == BTN_RIGHT {
                self.menu = Some(open_menu(&self.tree, x.max(0) as u32, y.max(0) as u32, row));
            } else if let Some(vi) = row {
                if let Some(path) = self.tree.activate(vi) {
                    self.open_path(&path);
                }
            }
            return Some(EventOutcome::Repaint);
        }

        if press && y >= tb_top && y < tb_bot && x >= px {
            match self.strip_spot(x, y) {
                Some(Spot::Tool(btn)) => self.toolbar_action(btn),
                Some(Spot::Close(idx)) => {
                    self.request_close(idx);
                }
                Some(Spot::Tab(idx)) => self.active = idx,
                None => {}
            }
            return Some(EventOutcome::Repaint);
        }

        let lit = act == Step::Repaint || explorer == Step::Repaint;
        lit.then_some(EventOutcome::Repaint)
    }

    // Apply a view-control click: 0 zoom out, 1 zoom in, 2 cycle the theme. The
    // theme is process-wide, so it is not tied to the active document.
    fn toolbar_action(&mut self, btn: usize) {
        use super::layout::{MAX_SCALE, MIN_SCALE};
        match btn {
            0 => {
                let doc = self.doc();
                doc.font_scale = doc.font_scale.saturating_sub(1).max(MIN_SCALE);
            }
            1 => {
                let doc = self.doc();
                doc.font_scale = (doc.font_scale + 1).min(MAX_SCALE);
            }
            _ => super::theme::cycle(),
        }
    }

    pub(super) fn activity_press(&mut self, target: Screen) -> EventOutcome {
        if self.screen == target {
            if target == Screen::Editor {
                self.sidebar_open = !self.sidebar_open;
                if self.sidebar_open {
                    self.tree.reload(self.owner_pid);
                }
            }
        } else {
            self.screen = target;
        }
        EventOutcome::Repaint
    }
}
