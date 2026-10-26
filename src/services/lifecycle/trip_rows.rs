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

//! Who is inside a trip lock (L81b): one writer, or readers by pid and
//! count; the first writer kept out, ahead of new readers while it keeps
//! asking; and who waits, to be woken when the lock comes free. Pure.

use alloc::vec::Vec;

/// How long a waiting writer's mark keeps new readers out unless it asks
/// again. One that has stopped asking, stopped or starved of the CPU, no
/// longer holds a free lock shut.
pub(crate) const FRESH_MS: u64 = 50;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct Rows {
    pub writer: Option<u32>,
    pub readers: Vec<(u32, u32)>,
    /// The first writer kept out, and when it last asked.
    pub next: Option<(u32, u64)>,
    pub waiting: Vec<u32>,
}

impl Rows {
    pub(crate) const fn new() -> Self {
        Rows { writer: None, readers: Vec::new(), next: None, waiting: Vec::new() }
    }

    /// The waiting writer whose mark still counts at `now`.
    pub(crate) fn next_at(&self, now: u64) -> Option<u32> {
        self.next.filter(|n| now.saturating_sub(n.1) <= FRESH_MS).map(|n| n.0)
    }

    /// Whether `me` may come in, as the writer when `whole`. A reader inside
    /// reads again past a waiting writer, or the two would wait on each other.
    pub(crate) fn may_take(&self, me: u32, whole: bool, now: u64) -> bool {
        let reads = self.readers.iter().any(|r| r.0 == me);
        let next = self.next_at(now);
        match (self.writer, whole) {
            (Some(_), _) => false,
            (None, true) => self.readers.is_empty() && next.is_none_or(|n| n == me),
            (None, false) => next.is_none() || reads,
        }
    }

    pub(crate) fn take(&mut self, me: u32, whole: bool) {
        self.waiting.retain(|&w| w != me);
        if whole {
            self.writer = Some(me);
            self.next = self.next.filter(|n| n.0 != me);
        } else if let Some(r) = self.readers.iter_mut().find(|r| r.0 == me) {
            r.1 += 1;
        } else {
            self.readers.push((me, 1));
        }
    }

    /// Whether no one is inside.
    pub(crate) fn free(&self) -> bool {
        self.writer.is_none() && self.readers.is_empty()
    }
}
