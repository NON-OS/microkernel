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

//! What the fuzz driver does to its rings around each round: where its
//! next command goes, noise ahead of a wait, a flush when the ring is
//! nearly full, and the checks every round ends with.

use super::driver::{Driver, RING_TRBS};
use super::hostile::hostile;
use super::rng::Rng;
use crate::controller::wait_command_completion::wait_command_completion;
use crate::event_ring::events::{command, CC_SUCCESS};
use crate::event_ring::fixture::TRB;
use crate::event_ring::producer::{ERDP_EHB_BIT, ERDP_FLAGS};
use crate::trb::commands::noop_command;

impl Driver {
    /// Where the next command on `cmd` goes: its ring wraps before the link.
    pub(super) fn next_command(&self) -> u64 {
        (self.cmd.crcr_value() & !0x3F) + (self.cmd_issued % RING_TRBS) * TRB
    }

    /// Hand back everything pending through a wait on a ring no noise names.
    pub(super) fn flush(&mut self) {
        let issued = self.flush.enqueue(noop_command(self.flush.cycle() != 0)).expect("noop");
        self.fx.hc.post(command(issued, CC_SUCCESS, 1));
        let done = wait_command_completion(self.fx.intr(), issued, &mut self.fx.ring);
        assert_eq!(done.map(|c| c.slot_id), Ok(1));
        assert_eq!(self.fx.hc.pending(), 0);
    }

    pub(super) fn noise(&mut self, rng: &mut Rng, issued: u64, dci: u8, request: u32) {
        for _ in 0..rng.below(5) {
            let event = hostile(rng, issued, dci, request);
            self.fx.hc.post(event);
        }
    }

    pub(super) fn check(&self) {
        let fx = &self.fx;
        let dequeue = fx.dequeue_bus();
        let erdp = fx.hc.erdp();
        assert_eq!(erdp & 0x7, 0, "DESI names the only segment");
        assert_eq!(erdp & !ERDP_FLAGS, dequeue, "ERDP names the next unread TRB");
        if fx.ring.drained_total() > 0 {
            assert_ne!(erdp & ERDP_EHB_BIT, 0, "EHB is cleared with every hand-back");
        }
    }
}
