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

//! A trip lock's rows as callers come and go (L81b): kept out, let go,
//! and swept of a pid that never runs again. Pure, for the host.

use alloc::vec::Vec;

use super::trip_rows::Rows;

impl Rows {
    /// Kept out: `me` waits. A writer marks itself next, or renews its
    /// mark, unless another writer's mark still counts.
    pub(crate) fn wait(&mut self, me: u32, whole: bool, now: u64) {
        if !self.waiting.contains(&me) {
            self.waiting.push(me);
        }
        if whole && self.next_at(now).is_none_or(|n| n == me) {
            self.next = Some((me, now));
        }
    }

    /// `me` lets go. Whether its row was there (a swept holder's is not),
    /// and, once no one is inside, who waits, to be woken after the rows
    /// are let go.
    pub(crate) fn give(&mut self, me: u32, whole: bool) -> (bool, Vec<u32>) {
        let had = if whole {
            let had = self.writer == Some(me);
            self.writer = self.writer.filter(|_| !had);
            had
        } else if let Some(r) = self.readers.iter_mut().find(|r| r.0 == me) {
            r.1 -= 1;
            self.readers.retain(|r| r.1 > 0);
            true
        } else {
            false
        };
        let woken = if self.free() { core::mem::take(&mut self.waiting) } else { Vec::new() };
        (had, woken)
    }

    /// Whether `me` is inside already. Kept out while inside, it would wait
    /// on itself for good: a writer asking again, or a reader asking to write.
    pub(crate) fn has(&self, me: u32) -> bool {
        self.writer == Some(me) || self.readers.iter().any(|r| r.0 == me)
    }

    /// Every row whose pid never runs again dropped, waiting ones too; true
    /// when the writer was one of them.
    pub(crate) fn sweep(&mut self, gone: impl Fn(u32) -> bool) -> bool {
        self.readers.retain(|r| !gone(r.0));
        self.waiting.retain(|&w| !gone(w));
        self.next = self.next.filter(|n| !gone(n.0));
        let writer = self.writer.is_some_and(&gone);
        self.writer = self.writer.filter(|_| !writer);
        writer
    }
}
