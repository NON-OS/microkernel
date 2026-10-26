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

//! Split view: Ctrl+Shift+D splits or joins, Ctrl+Shift+Left and Right or a
//! click move the keys between the panes (`term/split.rs`).

use nonos_app_skeleton::{EventOutcome as Out, InputEvent, InputKind, KEY_LEFT, KEY_RIGHT};
use nonos_app_skeleton::{MOD_CTRL, MOD_SHIFT};

use super::tabs::MAX_TABS;
use super::types::Terminal;
use crate::term::split::{retarget, right_of, sides};

const KEY_D: u32 = 0x44; // 'D', as it arrives with Shift held

impl Terminal {
    pub(super) fn split_command(&mut self, event: InputEvent) -> Option<Out> {
        if event.kind == InputKind::ButtonDown {
            return self.split_click(event.x);
        }
        let both = MOD_CTRL | MOD_SHIFT;
        if !event.is_key_down() || event.flags & both != both {
            return None;
        }
        match (event.code, self.split) {
            (KEY_D, _) => return Some(self.split_toggle()),
            (KEY_LEFT | KEY_RIGHT, Some(other)) => self.go(other),
            _ => return None,
        }
        Some(Out::Repaint)
    }

    /// Split beside the tab in front, or show one tab again.
    pub(super) fn split_toggle(&mut self) -> Out {
        match self.split {
            None if self.tabs.len() < MAX_TABS => {
                let was = self.active;
                self.open_tab();
                self.split = Some(was);
            }
            _ => self.split = None,
        }
        Out::Repaint
    }

    /// A click on the pane without the keys gives them to it, then goes on.
    fn split_click(&mut self, x: i32) -> Option<Out> {
        let other = self.split?;
        let (left, right) = sides(self.active, other);
        let over = if right_of(x, self.width) { right } else { left };
        if over != self.active {
            self.go(over);
        }
        None
    }

    /// Tab `new` to the front, both panes kept on screen.
    pub(super) fn go(&mut self, new: usize) {
        self.split = retarget(self.split, self.active, new);
        self.active = new;
    }
}
