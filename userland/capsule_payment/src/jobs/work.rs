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

//! The meter's one running job (plan/21, v1.4.14-15): its pure flow, the
//! one ASYNC request it waits on, and the job table that answers JOB.

use nonos_nox_meter::flow::{Contracts, Flow};
use nonos_nox_meter::job::Jobs;
use nonos_payment_proto::{EAGAIN, ETIME};

/// A step with no answer in this long fails its job (v1.4.15).
pub(super) const STEP_MS: u64 = 150_000;
/// How often the loop wakes to check that, while a step waits.
pub(super) const TICK_MS: u64 = 1_000;

pub(super) struct Pending {
    pub op: u16,
    pub req: u32,
    pub settle_pid: u32,
    pub sent_ms: u64,
}

pub(super) struct Running {
    pub id: u32,
    pub flow: Flow,
    pub contracts: Contracts,
    pub key_id: u32,
    pub address: [u8; 20],
    pub pending: Option<Pending>,
}

#[derive(Default)]
pub(crate) struct Work {
    pub jobs: Jobs,
    pub(super) run: Option<Running>,
    pub(super) next_req: u32,
    /// An offer's proof of work, walked in slices (solve.rs).
    pub(crate) solving: Option<alloc::boxed::Box<super::solving::Solving>>,
    /// The store-to-wallet mailbox (op 19).
    pub(crate) mailbox: nonos_nox_meter::mailbox::Mailbox,
}

impl Work {
    /// Fail the job whose step has waited past STEP_MS.
    pub fn tick(&mut self, now_ms: u64) {
        let sent = self.run.as_ref().and_then(|r| r.pending.as_ref()).map(|p| p.sent_ms);
        if sent.is_some_and(|t| now_ms.saturating_sub(t) >= STEP_MS) {
            self.finish(Err(ETIME));
        }
    }

    /// End the running job; a late answer to its step is then dropped.
    pub(super) fn finish(&mut self, r: Result<alloc::vec::Vec<u8>, i32>) {
        if let Some(run) = self.run.take() {
            // JOB's EAGAIN means "still running", so a job that ended on one
            // (settle's DELIVERED before any log, a host gone) ends ETIME.
            self.jobs.finish(run.id, r.map_err(|e| if e == EAGAIN { ETIME } else { e }));
        }
    }
}
