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

//! The meter's one running job: its pure flow, the
//! one ASYNC request it waits on, and the job table that answers JOB.

use nonos_nox_meter::flow::{Contracts, Flow};
use nonos_nox_meter::job::Jobs;
use nonos_nox_meter::timeout::step_timed_out;
use nonos_payment_proto::{EAGAIN, ETIME};

/// How often the loop wakes to check a waiting step's STEP_MS (timeout.rs).
pub(super) const TICK_MS: u64 = 1_000;

pub(super) struct Pending {
    pub op: u16,
    pub req: u32,
    pub settle_pid: u32,
    /// When the step was sent, on the monotonic clock (`meter::uptime_ms`).
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
    /// ORDER_DELIVERED's bad-chunk answers (v1.4.49).
    pub(crate) delivered: nonos_nox_meter::orders::KeptDelivered,
}

impl Work {
    /// Fail the job whose step has waited STEP_MS by `up_ms`, uptime.
    pub fn tick(&mut self, up_ms: u64) {
        let sent = self.run.as_ref().and_then(|r| r.pending.as_ref()).map(|p| p.sent_ms);
        if sent.is_some_and(|t| step_timed_out(t, up_ms)) {
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
