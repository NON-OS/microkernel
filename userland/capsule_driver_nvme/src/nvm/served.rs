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

//! Whose request a recovery's bound is for, and when it ends. It is the
//! kernel's (pid 0): only its request spends it, or, refused, passes it on
//! (bound::pass_on). Another sender's is held to it and leaves it, and any
//! other sender it served ends it with its next request: a sender has one
//! call out at a time, so that was sent after the reply, and the inbox is
//! FIFO, so every request queued while the recovery ran was taken first.

use super::queue::IoQueue;
use crate::error::{NvmeError, NvmeResult};

/// The kernel's pid: pid 0 is never handed to a process (medium_rule).
pub(super) const KERNEL: u32 = 0;

/// How many other senders a bound names as served. A fifth is not named, so
/// its next request ends nothing: the bound lasts until a named sender sends
/// again or the inbox is seen empty, later than it need, never sooner.
pub(super) const SERVED: usize = 4;

impl IoQueue {
    /// The bound `sender`'s request is held to, taken off the queue: none
    /// once a sender the bound served sends again.
    pub(super) fn held(&mut self, sender: u32) -> Option<u64> {
        let bound = self.answer_by.take();
        if sender != KERNEL && self.served.contains(&Some(sender)) {
            self.served = [None; SERVED];
            return None;
        }
        bound
    }

    /// `sender`'s request served under `bound`: the kernel's spent it, and
    /// another sender's leaves it, its sender named while there is room.
    pub(super) fn served_under(&mut self, sender: u32, bound: Option<u64>) {
        self.answer_by = bound.filter(|_| sender != KERNEL);
        if self.answer_by.is_none() {
            return;
        }
        if let Some(free) = self.served.iter_mut().find(|s| s.is_none()) {
            *free = Some(sender);
        }
    }

    /// Refused with nothing sent: the kernel's request passes its bound on,
    /// and another sender's leaves it. A refusal names no sender: its call
    /// may have timed out while the recovery ran, the request sent again.
    pub(super) fn refused(
        &mut self,
        sender: u32,
        bound: Option<u64>,
        e: NvmeError,
    ) -> NvmeResult<()> {
        if sender == KERNEL {
            self.pass_on(bound);
        } else {
            self.answer_by = bound;
        }
        Err(e)
    }
}
