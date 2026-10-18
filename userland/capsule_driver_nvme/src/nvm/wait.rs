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

use super::answer_by::wait_budget;
use super::park::{parked, Park};
use super::queue::IoQueue;
use super::ring_wait::ring_wait;
use super::settle_rule::settled;
use crate::clock::{self, Budget};
use crate::error::{NvmeError, NvmeResult};
use crate::regs::Regs;

impl IoQueue {
    /// Wait for `cid` as long as the request in progress allows (answer_by);
    /// should that run out, the command is still the controller's.
    pub(super) fn wait(&mut self, regs: Regs, cid: u16) -> NvmeResult<()> {
        let (ring, doorbell, budget) = (self.cq.user_va(), self.cq_db, wait_budget(self.answer_by));
        let done =
            ring_wait(&mut self.cursor, ring, doorbell, regs, cid, parked(budget, &mut self.park));
        if matches!(done, Err(NvmeError::ControllerTimeout)) {
            self.out = Some(cid);
        }
        done
    }

    /*
     * A command given up on is waited out before the data buffer is filled
     * or another command goes in: a write still in flight could otherwise
     * take the next write's bytes to its own sectors, and a read still in
     * flight could land in the buffer under the next read's answer. Refused,
     * buffer untouched, while it stays out (settle_rule).
     */
    pub fn settle(&mut self, regs: Regs) -> NvmeResult<()> {
        let answer_by = self.answer_by;
        self.settle_by(regs, move || wait_budget(answer_by))
    }

    /// `settle`, waiting at most `ms`: recovery gives the command a short
    /// grace after an Abort, or one look at the ring (0), before a reset.
    pub(crate) fn settle_within(&mut self, regs: Regs, ms: u64) -> NvmeResult<()> {
        self.settle_by(regs, move || clock::budget(ms))
    }

    /// Settle on a budget the clock is read for only once something is out.
    fn settle_by<B: FnOnce() -> Budget>(&mut self, regs: Regs, budget: B) -> NvmeResult<()> {
        let (ring, doorbell) = (self.cq.user_va(), self.cq_db);
        let (cursor, park) = (&mut self.cursor, &mut self.park);
        settled(&mut self.out, |cid| {
            ring_wait(cursor, ring, doorbell, regs, cid, parked(budget(), park))
        })
    }

    /// The command given up on that the controller may still be moving.
    pub(crate) fn stuck(&self) -> Option<u16> {
        self.out
    }

    /// Park the queue's waits on MSI-X grant `grant` (0: none bound).
    pub fn park_on(&mut self, grant: u64) {
        self.park = Park::on(grant);
    }
}
