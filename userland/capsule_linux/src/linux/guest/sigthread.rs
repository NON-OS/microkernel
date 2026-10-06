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

//! What each thread has of its own: the signals it blocks, its alternate
//! stack, and the mask a sigsuspend put aside until a handler returns.

use super::sigalt::NONE;
use super::sigqueue::Signals;
use super::sigstate::blockable;

#[derive(Clone, Copy)]
pub struct ThreadSig {
    pub tid: u32,
    pub blocked: u64,
    /// ss_sp, ss_flags, ss_size.
    pub alt: [u64; 3],
    /// The mask sigsuspend replaced, which the handler's frame restores.
    pub saved: Option<u64>,
}

impl ThreadSig {
    pub(super) fn new(tid: u32, blocked: u64) -> Self {
        ThreadSig { tid, blocked, alt: NONE, saved: None }
    }
}

impl Signals {
    pub fn thread(&mut self, tid: u32) -> &mut ThreadSig {
        let at = match self.threads.iter().position(|t| t.tid == tid) {
            Some(at) => at,
            None => {
                self.threads.push(ThreadSig::new(tid, 0));
                self.threads.len() - 1
            }
        };
        &mut self.threads[at]
    }

    pub fn blocked(&self, tid: u32) -> u64 {
        self.threads.iter().find(|t| t.tid == tid).map_or(0, |t| t.blocked)
    }

    pub fn set_blocked(&mut self, tid: u32, mask: u64) {
        self.thread(tid).blocked = blockable(mask);
    }

    /// A new thread starts with its creator's mask and no alternate stack,
    /// as clone(CLONE_VM) leaves it.
    pub fn born(&mut self, parent: u32, child: u32) {
        let blocked = self.blocked(parent);
        self.threads.retain(|t| t.tid != child);
        self.threads.push(ThreadSig::new(child, blocked));
    }
}
