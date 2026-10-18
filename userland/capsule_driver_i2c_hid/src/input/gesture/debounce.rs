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
use super::types::{TouchActions, TouchGesture};

impl TouchGesture {
    /// A physical clickpad press maps to the left button, palm or not: a
    /// deliberate press is a deliberate press. Debounced: two consecutive
    /// samples must agree before an edge is emitted, so a single-frame bit
    /// flip (torn read, misplaced field) can never click.
    pub(super) fn debounce(&mut self, button: bool, act: &mut TouchActions) {
        if button == self.was_button {
            self.button_run = 0;
            return;
        }
        self.button_run = self.button_run.saturating_add(1);
        if self.button_run >= 2 {
            if button {
                act.button_down = true;
            } else {
                act.button_up = true;
            }
            self.was_button = button;
            self.button_run = 0;
        }
    }
}
