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

//! The drop-down's keys as the window takes them (`term/drop.rs`).

use nonos_app_skeleton::{EventOutcome, InputEvent};

use super::types::Terminal;

impl Terminal {
    /// F12, or Esc while dropped with nothing else holding the keys.
    pub(super) fn drop_key(&mut self, e: InputEvent) -> Option<EventOutcome> {
        if !e.is_key_down() {
            return None;
        }
        let s = self.cur_ref();
        let quiet = s.search.is_none() && s.find.is_none() && s.theme_pick.is_none();
        let idle = quiet && !s.fg_running && s.line.as_bytes().is_empty();
        self.dropped = crate::term::drop::key(self.dropped, e.code, e.flags, idle)?;
        Some(EventOutcome::Repaint)
    }
}
