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

//! Lending the family's pipes, eventfd counters and timerfd timers to the
//! guest being answered.
//!
//! All three are the family's, since a fork leaves their descriptors in more
//! than one process. The guest being answered holds them for that one answer,
//! with a note of which ends are still open anywhere in the family: end of
//! file, a broken pipe and a hang-up are all decided by that note.

use alloc::vec::Vec;
use core::mem;

use super::family::Family;
use crate::linux::call::tasks::count;
use crate::linux::guest::Kind;

impl Family {
    pub(super) fn lend(&mut self, i: usize) {
        let ends = self.pipe_ends();
        let events = self.named(Kind::Event, self.events.len());
        let timers = self.named(Kind::Timer, self.timers.len());
        let tasks = count(self.guests.iter().map(|g| (g.threads.len(), g.ended.len())));
        let g = &mut self.guests[i];
        g.pipe_ends = ends;
        g.event_used = events;
        g.timer_used = timers;
        g.tasks = tasks;
        mem::swap(&mut self.pipes, &mut g.pipes);
        mem::swap(&mut self.events, &mut g.events);
        mem::swap(&mut self.timers, &mut g.timers);
    }

    pub(super) fn take_back(&mut self, i: usize) {
        let g = &mut self.guests[i];
        mem::swap(&mut self.pipes, &mut g.pipes);
        mem::swap(&mut self.events, &mut g.events);
        mem::swap(&mut self.timers, &mut g.timers);
        g.pipe_ends = Vec::new();
        g.event_used = Vec::new();
        g.timer_used = Vec::new();
    }

    /// For each of `len` slots, whether a descriptor of `kind` anywhere in
    /// the family names it.
    fn named(&self, kind: Kind, len: usize) -> Vec<bool> {
        let mut used = alloc::vec![false; len];
        let open = self.guests.iter().flat_map(|g| g.fds.iter());
        for f in open.filter(|f| f.kind == kind) {
            if let Some(u) = used.get_mut(f.handle as usize) {
                *u = true;
            }
        }
        used
    }

    /// For each pipe: whether a read end is open, whether a write end is.
    fn pipe_ends(&self) -> Vec<(bool, bool)> {
        let mut ends = alloc::vec![(false, false); self.pipes.len()];
        let open = self.guests.iter().flat_map(|g| g.fds.iter());
        for f in open.filter(|f| f.kind == Kind::Pipe) {
            if let Some(end) = ends.get_mut(f.handle as usize) {
                match f.writable {
                    true => end.1 = true,
                    false => end.0 = true,
                }
            }
        }
        ends
    }
}
