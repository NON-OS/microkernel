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

//! Split view: Ctrl+Shift+D opens a new tab beside the one in front, or
//! closes the split; Ctrl+Shift+Left and Right move the keys between the two
//! panes; a click on a pane gives it the keys (`term/split.rs`).

use nonos_app_skeleton::{
    EventOutcome, InputEvent, InputKind, KEY_LEFT, KEY_RIGHT, MOD_CTRL, MOD_SHIFT,
};

use super::tabs::MAX_TABS;
use super::types::Terminal;
use crate::term::split::{retarget, right_of, sides};

/// 'D' as it arrives with Shift held.
const KEY_D: u32 = 0x44;

impl Terminal {
    pub(super) fn split_command(&mut self, event: InputEvent) -> Option<EventOutcome> {
        if event.kind == InputKind::ButtonDown {
            return self.split_click(event.x);
        }
        let both = MOD_CTRL | MOD_SHIFT;
        if !event.is_key_down() || event.flags & both != both {
            return None;
        }
        match (event.code, self.split) {
            (KEY_D, None) if self.tabs.len() < MAX_TABS => {
                let was = self.active;
                self.open_tab();
                self.split = Some(was);
            }
            (KEY_D, Some(_)) => self.split = None,
            (KEY_LEFT | KEY_RIGHT, Some(other)) => self.go(other),
            _ => return None,
        }
        Some(EventOutcome::Repaint)
    }

    /// A click on the pane without the keys gives them to it, and goes on to
    /// the pane as a click.
    fn split_click(&mut self, x: i32) -> Option<EventOutcome> {
        let other = self.split?;
        let (left, right) = sides(self.active, other);
        let over = if right_of(x, self.width) { right } else { left };
        if over != self.active {
            self.go(over);
        }
        None
    }

    /// Bring tab `new` to the front, keeping both panes on screen.
    pub(super) fn go(&mut self, new: usize) {
        self.split = retarget(self.split, self.active, new);
        self.active = new;
    }
}
