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

//! A toast that says a state, put up in place of the ones it supersedes.

use super::super::toast::UptimeMs;
use super::super::NotifyLevel;
use super::queue::{ToastQueue, MAX_TOASTS};

impl ToastQueue {
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
}
