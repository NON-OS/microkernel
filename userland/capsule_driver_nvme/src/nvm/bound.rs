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

//! The bound a recovery leaves for the kernel's request taken after it.
//! The kernel queues its next request as its reply reaches it, and stops
//! waiting for a request 35 s after it queued it (src/services/lifecycle/
//! transport.rs, reply_wait.rs); the recovery after that reply runs on the
//! same clock. So a request already queued when it is taken is answered
//! within REPLY_BY_MS of that reply, or refused with nothing sent
//! (answer_by), after a recovery that followed the kernel's own request or
//! another sender's stall. The bound lasts until the inbox is seen empty
//! (server::take) or another sender it served sends again (served). That
//! is a guess at what the kernel knows; the plan's K14 gives each message
//! its kernel-stamped timestamp_ms and a true try-receive, and D5b then
//! bounds each request from that enqueue time instead. Until then it misses
//! a kernel request queued before another sender's stalled one was taken,
//! and a sender with two requests queued at once; a sender resending each
//! refusal at once, or a fifth sender served, keeps the bound, and requests
//! are refused; and a non-I/O request never touches it.

use super::constants::{KERNEL_WAIT_MS, REPLY_BY_MS};
use super::queue::IoQueue;
use super::served::{KERNEL, SERVED};
use crate::clock;

impl IoQueue {
    /// Bound the next request taken to REPLY_BY_MS from this reply, as a
    /// recovery starts, or keep the bound pending if it is sooner; no sender
    /// is served under it yet. After another `sender`'s stall it counts from
    /// when that request was taken (`taken`), as a kernel request queued
    /// during its wait came after that; with no clock, too, `taken` coming
    /// before the reply.
    pub(crate) fn bound_next(&mut self, sender: u32) {
        let now = if sender == KERNEL { clock::uptime_ms() } else { None };
        let own = now.unwrap_or(self.taken).saturating_add(REPLY_BY_MS);
        self.answer_by = Some(self.answer_by.map_or(own, |by| by.min(own)));
        self.served = [None; SERVED];
    }

    /// Whether a bound waits for the request taken next.
    pub fn bound_pending(&self) -> bool {
        self.answer_by.is_some()
    }

    /// The inbox was seen empty: whatever comes next was queued fresh.
    pub fn inbox_empty(&mut self) {
        (self.answer_by, self.served) = (None, [None; SERVED]);
    }

    /// The kernel's request, refused because its `bound` passed. The kernel
    /// stopped waiting for it no later than KERNEL_WAIT_MS after the bound's
    /// reply, and only then queued its next, so the next request taken
    /// before the inbox is seen empty is bounded KERNEL_WAIT_MS on.
    pub(super) fn pass_on(&mut self, bound: Option<u64>) {
        self.answer_by = bound.map(|by| by.saturating_add(KERNEL_WAIT_MS));
    }
}
