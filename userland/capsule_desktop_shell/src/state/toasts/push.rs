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

//! Toasts put up: a notice, or a held one.

use super::super::toast::{Toast, UptimeMs, TOAST_HELD_MS, TOAST_LIFETIME_MS};
use super::super::NotifyLevel;
use super::queue::{ToastQueue, MAX_TOASTS};

impl ToastQueue {
    /// A transient notice, up for `TOAST_LIFETIME_MS`.
    pub fn push(&mut self, text: &[u8], level: NotifyLevel, now: UptimeMs) {
        self.push_for(text, level, now, TOAST_LIFETIME_MS);
    }

    /// A notice held up for `TOAST_HELD_MS` by design (the store's health,
    /// said once per boot). Bounded all the same, and dismissed by a press.
    pub fn push_held(&mut self, text: &[u8], level: NotifyLevel, now: UptimeMs) {
        self.push_for(text, level, now, TOAST_HELD_MS);
    }

    /// A notice still up says it already: the repeat is dropped, and the
    /// one shown keeps the expiry of its first showing. Re-arming it on every
    /// repeat ("Terminal opened" on each of a run of quick launches) would
    /// keep it up for as long as the repeats came.
    fn push_for(&mut self, text: &[u8], level: NotifyLevel, now: UptimeMs, lifetime_ms: i64) {
        self.expire(now);
        if self.entries.iter().flatten().any(|t| t.says(text, level)) {
            return;
        }
        // Marked, not played: the tone goes out on the clock tick, so no drag
        // handler waits on the audio service to finish a toast.
        crate::sound::mark(level);
        let toast = Toast::new(text, level, now, lifetime_ms);
        self.generation = self.generation.wrapping_add(1);
        if let Some(slot) = self.entries.iter_mut().find(|e| e.is_none()) {
            *slot = Some(toast);
            return;
        }
        self.entries.rotate_left(1);
        self.entries[MAX_TOASTS - 1] = Some(toast);
    }
}
