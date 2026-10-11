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

//! Every process this personality hosts: the guest it started and whatever
//! that guest forks. Each keeps its own descriptors, break and cwd. Pipe
//! buffers are the family's, lent to the guest being answered and taken back
//! (`family_lend`): a pipe opened before a fork has its ends in different
//! processes. A trap is answered in family_answer.rs.

use alloc::vec::Vec;
use core::mem;

use super::pid_ns::PidNs;
use crate::linux::guest::{Event, Guest, Timer};

pub struct Family {
    pub(super) guests: Vec<Guest>,
    pub(super) pipes: Vec<Vec<u8>>,
    pub(super) events: Vec<Event>,
    pub(super) timers: Vec<Timer>,
    pub(super) root: u32,
    pub(super) root_code: i32,
    pub(super) ns: PidNs,
}

impl Family {
    pub fn new(mut first: Guest) -> Self {
        let (pipes, root) = (mem::take(&mut first.pipes), first.pid);
        let events = mem::take(&mut first.events);
        let timers = mem::take(&mut first.timers);
        let ns = PidNs::new(first.parent, root);
        Family { guests: alloc::vec![first], pipes, events, timers, root, root_code: 0, ns }
    }

    /// Done once nothing it hosts is left; the code is the first guest's.
    pub fn done(&self) -> Option<i32> {
        self.guests.is_empty().then_some(self.root_code)
    }
}
