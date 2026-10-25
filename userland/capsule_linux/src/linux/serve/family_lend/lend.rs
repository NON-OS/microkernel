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

//! The family's pipes, eventfd counters and timerfd timers lent to the guest
//! being answered, with the note of what is open anywhere, and taken back.

use alloc::vec::Vec;
use core::mem;

use super::super::family::Family;
use crate::linux::call::tasks::count;
use crate::linux::guest::Kind;

impl Family {
    pub(in super::super) fn lend(&mut self, i: usize) {
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

    pub(in super::super) fn take_back(&mut self, i: usize) {
        let g = &mut self.guests[i];
        mem::swap(&mut self.pipes, &mut g.pipes);
        mem::swap(&mut self.events, &mut g.events);
        mem::swap(&mut self.timers, &mut g.timers);
        g.pipe_ends = Vec::new();
        g.event_used = Vec::new();
        g.timer_used = Vec::new();
    }
}
