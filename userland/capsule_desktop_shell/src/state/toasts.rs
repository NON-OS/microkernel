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

use super::toast::Toast;
use super::NotifyLevel;

pub const MAX_TOASTS: usize = 3;
pub struct ToastQueue {
    entries: [Option<Toast>; MAX_TOASTS],
}

impl ToastQueue {
    pub const fn new() -> Self {
        Self { entries: [None; MAX_TOASTS] }
    }

    pub fn push(&mut self, text: &[u8], level: NotifyLevel, now_ms: i64) {
        // Marked, not played: the tone goes out on the clock tick, so no drag
        // handler waits on the audio service to finish a toast.
        crate::sound::mark(level);
        let toast = Toast::new(text, level, now_ms);
        if let Some(slot) = self.entries.iter_mut().find(|e| e.is_none()) {
            *slot = Some(toast);
            return;
        }
        self.entries.rotate_left(1);
        self.entries[MAX_TOASTS - 1] = Some(toast);
    }

    pub fn expire(&mut self, now_ms: i64) -> bool {
        let mut changed = false;
        for slot in self.entries.iter_mut() {
            if slot.is_some_and(|t| t.expires_at_ms <= now_ms) {
                *slot = None;
                changed = true;
            }
        }
        changed
    }

    pub fn iter_live(&self) -> impl Iterator<Item = &Toast> {
        self.entries.iter().filter_map(|e| e.as_ref())
    }

    pub fn is_empty(&self) -> bool {
        self.entries.iter().all(|e| e.is_none())
    }
}
