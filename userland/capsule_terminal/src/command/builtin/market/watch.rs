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

//! The installs this Terminal asked for and has not said the end of, asked
//! about at most once a second; apart so terminal_line_proofs holds it.

use alloc::vec::Vec;

pub const MAX: usize = 8;
pub const EVERY_MS: i64 = 1000;

/// Each listing with a tag of the caller's (which of install or uninstall).
pub struct Watch<K> {
    ids: Vec<(Vec<u8>, K)>,
    next_ms: i64,
}

impl<K: Copy> Default for Watch<K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Copy> Watch<K> {
    pub const fn new() -> Self {
        Self { ids: Vec::new(), next_ms: 0 }
    }

    /// Follow `id` anew. When full, the first asked for gives way.
    pub fn add(&mut self, id: &[u8], tag: K) {
        self.ids.retain(|(have, _)| have != id);
        if self.ids.len() >= MAX {
            self.ids.remove(0);
        }
        self.ids.push((id.to_vec(), tag));
    }

    /// A second after the last round, ask each its stage; those `done` says
    /// finished are handed back and followed no more. With none, no clock.
    pub fn due<S: Copy>(
        &mut self,
        now_ms: impl FnOnce() -> i64,
        mut ask: impl FnMut(&[u8]) -> S,
        done: impl Fn(S) -> bool,
    ) -> Vec<(Vec<u8>, K, S)> {
        let now = if self.ids.is_empty() { i64::MIN } else { now_ms() };
        if now < self.next_ms {
            return Vec::new();
        }
        self.next_ms = now.saturating_add(EVERY_MS);
        let mut ended = Vec::new();
        self.ids.retain(|(id, tag)| {
            let stage = ask(id);
            let finished = done(stage);
            if finished {
                ended.push((id.clone(), *tag, stage));
            }
            !finished
        });
        ended
    }
}
