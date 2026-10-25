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

//! Toasts going: their time up, or the panel pressed.

use super::super::toast::UptimeMs;
use super::queue::{ToastQueue, MAX_TOASTS};

impl ToastQueue {
    pub fn expire(&mut self, now: UptimeMs) -> bool {
        let mut changed = false;
        for slot in self.entries.iter_mut() {
            if slot.is_some_and(|t| t.over(now)) {
                *slot = None;
                changed = true;
            }
        }
        if changed {
            self.generation = self.generation.wrapping_add(1);
        }
        changed
    }

    /// Dismiss every toast: a press on the panel puts it away.
    pub fn clear(&mut self) {
        if !self.is_empty() {
            self.generation = self.generation.wrapping_add(1);
        }
        self.entries = [None; MAX_TOASTS];
    }
}
