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

use super::toast::{Toast, UptimeMs, TOAST_HELD_MS, TOAST_LIFETIME_MS};
use super::NotifyLevel;

pub const MAX_TOASTS: usize = 3;
pub struct ToastQueue {
    entries: [Option<Toast>; MAX_TOASTS],
    /// Bumped on every change to what the panel shows, so the chrome is
    /// repainted when, and only when, its toasts are out of date.
    generation: u32,
}

impl ToastQueue {
    pub const fn new() -> Self {
        Self { entries: [None; MAX_TOASTS], generation: 0 }
    }

    /// A transient notice, up for `TOAST_LIFETIME_MS`.
    pub fn push(&mut self, text: &[u8], level: NotifyLevel, now: UptimeMs) {
        self.push_for(text, level, now, TOAST_LIFETIME_MS);
    }

    /// A notice held up for `TOAST_HELD_MS` by design (the store's health,
    /// said once per boot). Bounded all the same, and dismissed by a press.
    pub fn push_held(&mut self, text: &[u8], level: NotifyLevel, now: UptimeMs) {
        self.push_for(text, level, now, TOAST_HELD_MS);
    }

    /// A transient notice that says a state (the volume), put up in place of
    /// every toast up whose text `supersedes` accepts. A run of changes shows
    /// the last one, up for its whole lifetime from it, instead of a stack of
    /// the values it passed through. The toasts it keeps keep their order.
    pub fn replace(
        &mut self,
        text: &[u8],
        level: NotifyLevel,
        now: UptimeMs,
        supersedes: impl Fn(&[u8]) -> bool,
    ) {
        let mut kept = [None; MAX_TOASTS];
        let mut n = 0;
        for toast in self.entries.iter().flatten() {
            if !supersedes(&toast.text[..toast.len]) {
                kept[n] = Some(*toast);
                n += 1;
            }
        }
        if n != self.entries.iter().flatten().count() {
            self.entries = kept;
            self.generation = self.generation.wrapping_add(1);
        }
        self.push(text, level, now);
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

    /// When the next toast's time is up, so the serve loop wakes for it
    /// whether or not anything else happens. None with nothing up.
    pub fn next_expiry(&self) -> Option<UptimeMs> {
        self.entries.iter().flatten().map(|t| UptimeMs(t.expires_at_ms)).min()
    }

    pub fn iter_live(&self) -> impl Iterator<Item = &Toast> {
        self.entries.iter().filter_map(|e| e.as_ref())
    }

    /// Dismiss every toast: a press on the panel puts it away.
    pub fn clear(&mut self) {
        if !self.is_empty() {
            self.generation = self.generation.wrapping_add(1);
        }
        self.entries = [None; MAX_TOASTS];
    }

    /// Changes with every push, expiry and dismissal, never otherwise.
    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn is_empty(&self) -> bool {
        self.entries.iter().all(|e| e.is_none())
    }
}

impl Default for ToastQueue {
    fn default() -> Self {
        Self::new()
    }
}
