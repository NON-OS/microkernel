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

//! Every process this personality hosts: the guest it started, and whatever
//! that guest forks.
//!
//! Each has its own state, so a child's descriptors, break and cwd are its
//! own. Pipe buffers are the family's, since a pipe opened before a fork has
//! a reader and a writer in different processes; they are lent to the guest
//! being answered and taken back after.

use alloc::vec::Vec;
use core::mem;

use nonos_libc::{mk_foreign_reply, ForeignFrame};

use super::answer::Answer;
use super::dispatch::answer;
use crate::linux::guest::Guest;

pub struct Family {
    pub(super) guests: Vec<Guest>,
    pub(super) pipes: Vec<Vec<u8>>,
    pub(super) root: u32,
    pub(super) root_code: i32,
}

impl Family {
    pub fn new(mut first: Guest) -> Self {
        let pipes = mem::take(&mut first.pipes);
        let root = first.pid;
        Family { guests: alloc::vec![first], pipes, root, root_code: 0 }
    }

    pub fn answer(&mut self, frame: &ForeignFrame) {
        let Some(g) = self.guests.iter_mut().find(|g| g.owns(frame.pid)) else {
            return;
        };
        mem::swap(&mut self.pipes, &mut g.pipes);
        let got = answer(g, frame);
        mem::swap(&mut self.pipes, &mut g.pipes);
        let born = mem::take(&mut g.forked);
        if let Answer::Reply(value) = got {
            let _ = mk_foreign_reply(frame.pid, value);
        }
        self.guests.extend(born);
    }

    /// Done once nothing it hosts is left; the code is the first guest's.
    pub fn done(&self) -> Option<i32> {
        self.guests.is_empty().then_some(self.root_code)
    }
}
